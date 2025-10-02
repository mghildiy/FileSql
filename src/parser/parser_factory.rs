use crate::parser::entities::parser::Parse;
use crate::parser::entities::csv_parser::CsvParser;
use crate::parser::entities::file_types::FileType;
use crate::parser::entities::json_parser::JSONParser;

fn get_parser(file_type: &FileType) -> Box<dyn Parse> {
    match file_type {
        FileType::CSV => Box::new(CsvParser {}),
        FileType::JSON => Box::new(JSONParser {})
    }
}