use std::fs::File;
use std::io::{BufReader, Lines};
use chrono::NaiveDate;
use crate::parser::entities::entities::Value;
use crate::parser::entities::Record;

pub struct RecordIterator {
    lines: Lines<BufReader<File>>,
    date_format: Option<String>,
}

pub fn create_record_iterator(lines: Lines<BufReader<File>>, date_format: Option<String>) -> RecordIterator {
    RecordIterator {
        lines: lines,
        date_format: date_format
    }
}

impl Iterator for RecordIterator {
    type Item = Record;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(line) = self.lines.next() {
            match line {
                Ok(line) => {
                    return Some(self.line_to_record(line));
                },
                Err(e) => {
                    eprintln!("Skipping line due to error: {}", e);
                    continue
                }
            }
        }
        None
    }
}

impl RecordIterator {
    fn line_to_record(&self, line: String) -> Record {
        let raw_values: Vec<String> = line.split(",")
            .map(|s| s.trim().to_string())
            .collect();
        let values: Vec<Value> = raw_values.iter()
            .map(|raw_value| to_value(&raw_value, &self.date_format))
            .collect();

        Record { columns: values }
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
    } else if raw.trim().is_empty() {
        Value::Null
    } else {
        Value::String(raw.to_string())
    }
}

fn is_valid_date(date_format: &Option<String>, input: &str) -> Option<NaiveDate> {
    match date_format {
        Some(date_format) => {
            let possible_date = NaiveDate::parse_from_str(input, date_format);
            match possible_date{
                Ok(date) => Some(date),
                Err(e) => {
                    eprintln!("{} is not a valid date: {}", input, e);
                    None
                }
            }
        },
        None => None
    }
}