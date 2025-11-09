use crate::parser::entities::parser::Parse;
use crate::parser::entities::csv_parser::CsvParser;
use crate::parser::entities::file_types::FileType;
use crate::parser::entities::json_parser::JSONParser;
use crate::parser::error::ParserError;

pub fn get_parser(file_type: &FileType) -> Result<Box<dyn Parse>, ParserError> {
    match file_type {
        FileType::CSV => Ok(Box::new(CsvParser {})),
        FileType::JSON => Ok(Box::new(JSONParser {})),
        _ => Err(ParserError{message: "Unsupported file type".to_string()})
    }
}