use crate::parser::entities::entities::Value;
use crate::parser::entities::parser::{Parse, RecordIterator};
use crate::parser::entities::{entities, Record};
use crate::parser::error::ParserError;
use chrono::NaiveDate;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

pub struct CsvParser {
    pub(crate) date_format: Option<String>
}

impl Parse for CsvParser {
    fn parse(&mut self, file_path: &str) -> Result<RecordIterator, ParserError> {
        //todo!()
        match self.lines_to_records(file_path) {
            Ok(records) => Ok(Box::new(records.into_iter()) as Box<dyn Iterator<Item = Record>>),
            Err(error) => Err(error)
        }
    }
}

impl CsvParser {
    fn lines_to_records(&self, file_path: &str) -> Result<Vec<Record>, ParserError> {
        match get_file_handler(file_path) {
            Ok(reader) => {
                self.extract_records(reader)
            },
            Err(error) => return Err(error)
        }
    }

    fn extract_records(&self, mut reader: BufReader<File>) -> Result<Vec<Record>, ParserError> {
        let maybe_first_line = read_first_line(&mut reader);
        match maybe_first_line {
            Some(first_line) => {
                let mut records = Vec::new();
                for line in reader.lines() {
                    match line {
                        Ok(line) => {
                            let raw_values: Vec<String> = line.split(",")
                                .map(|s| s.trim().to_string())
                                .collect();
                            let values: Vec<Value> = raw_values.iter()
                                .map(|raw_value| to_value(&raw_value, &self.date_format))
                                .collect();
                            let record = Record {columns: values};
                            records.push(record);
                        },
                        Err(e) => continue
                    }
                }
                Ok(records)
            },
            None => return Err(ParserError{message: format!("File is empty")})
        }
    }

    fn line_to_record(&self, columns: Vec<String>,line: &str) -> Record {
        let cols_values: Vec<Value> = line.split(",")
            .enumerate()
            .map(|(index, subpart)| self.subpart_to_value(&columns[index] ,subpart.to_string()))
            .collect();
        entities::create(cols_values)
    }

    fn subpart_to_value(&self, column: &str, subpart: String) -> Value {
        to_value(&*subpart, &self.date_format)
    }
}

fn to_value (raw: &str, date_format: &Option<String>) -> Value {
    if let Ok(int_value) = raw.parse::<i64>() {
        Value::Int(int_value)
    } else if let Ok(float_value) = raw.parse::<f64>() {
        Value::Float(float_value)
    } else  if raw.eq_ignore_ascii_case("true") {
        Value::Bool(true)
    } else if raw.eq_ignore_ascii_case("false") {
        Value::Bool(false)
    } else if let Some(date) = is_valid_date(date_format, raw) {
        Value::Date(date)
    } else {
        Value::String(raw.to_string())
    }
}

fn is_valid_date(date_format: &Option<String>, input: &str) -> Option<NaiveDate> {
    match date_format {
        Some(date_format) => {
            let possible_date = NaiveDate::parse_from_str(input, date_format);
            if possible_date.is_ok() {
                Some(possible_date.unwrap())
            } else {
                None
            }
        },
        None => None
    }
}

fn get_file_handler(file_path: &str) -> Result<BufReader<File>, ParserError> {
    match File::open(file_path) {
        Ok(file) => Ok(BufReader::new(file)),
        Err(e) => {
            eprintln!("Error while opening file: {}", e);
            Err(ParserError{ message: format!("Unable to open file {file_path}")})
        }
    }
}

fn get_columns_and_file(path: &str) -> Option<(Vec<String>, BufReader<File>)>  {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            println!("Error opening file: {}", e);
            return None
        },
    };
    let mut reader = BufReader::new(file);
    let columns = read_first_line(&mut reader)?;
    Some((columns, reader))
}

fn read_first_line(reader: &mut BufReader<File>) -> Option<Vec<String>> {
    let mut first_line = String::new();
    if reader.read_line(&mut first_line).ok()? > 0 {
        let cols = first_line.trim_end().split(",").map(|col| col.to_string()).collect();
        Some(cols)
    } else {
        None
    }
}