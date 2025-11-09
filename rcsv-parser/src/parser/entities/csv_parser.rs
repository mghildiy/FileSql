use crate::parser::entities::parser::Parse;
use crate::parser::entities::record_iterator::{create_record_iterator, RecordIterator};
use crate::parser::error::ParserError;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct CsvParser {}

impl Parse for CsvParser {
    fn parse(&mut self, file_path: &str, date_format: Option<String>) -> Result<(RecordIterator, Option<Vec<String>>), ParserError> {
        record_iterator(file_path, date_format)
    }
}

fn record_iterator(file_path: &str, date_format: Option<String>) -> Result<(RecordIterator, Option<Vec<String>>), ParserError> {
    match get_file_handler(file_path) {
        Ok(mut reader) => {
            let maybe_first_line = read_first_line(&mut reader);
            match maybe_first_line {
                Some(first_line) => {
                    let record_iterator = create_record_iterator(reader.lines(), date_format);
                    Ok((record_iterator, Some(first_line)))
                },
                None => Err(ParserError{message: "File is empty".to_string() })
            }
        },
        Err(error) => return Err(error)
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

fn read_first_line(reader: &mut BufReader<File>) -> Option<Vec<String>> {
    let mut first_line = String::new();
    if reader.read_line(&mut first_line).ok()? > 0 {
        let cols = first_line.trim_end().split(",").map(|col| col.to_string()).collect();
        Some(cols)
    } else {
        None
    }
}