use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader, Read};
use chrono::NaiveDate;
use crate::parser::entities::entities::{Value, KV};
use crate::parser::entities::parser::Parse;
use crate::parser::entities::{entities, Record};
use crate::parser::error::ParserError;

pub struct CsvParser {
    date_format: Option<String>
}

impl Parse for CsvParser {
    fn parse(&self, path: &str) -> Result<Box<dyn Iterator<Item=Record>>, ParserError> {
        todo!()
        //read_to_string(path).unwrap().lines()
    }
}

impl CsvParser {

    fn lines_to_records(file: &str) -> Vec<Record> {
        match get_columns_and_file(file) {
            Some((columns, reader)) => {
                //columns.iter().map(|line| line_to_record(columns))
                reader.lines()
                    .map(|line| line_to_record(columns, &*line?))
            }
            None => Vec::new()
        }
    }

    fn line_to_record(&self, columns: Vec<String>,line: &str) -> Record {
        let cols_values: Vec<KV> = line.split(",")
            .enumerate()
            .map(|(index, subpart)| self.subpart_to_kv(&columns[index] ,subpart.to_string()))
            .collect();
        entities::create(cols_values)
    }

    fn subpart_to_kv(&self, col: &str, part: String) -> KV {
        KV {
            key: col,
            value: to_value(&*part, &self.date_format)
        }
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

fn is_valid_date(date_format: Option<&str>, input: &str) -> Option<NaiveDate> {
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