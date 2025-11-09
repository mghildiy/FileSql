use crate::parser::entities::parser::Parse;
use crate::parser::entities::record_iterator::RecordIterator;
use crate::parser::error::ParserError;

pub struct JSONParser {}

impl Parse for JSONParser {
    fn parse(&mut self, file_path: &str, date_format: Option<String>) -> Result<(RecordIterator, Option<Vec<String>>), ParserError> {
        todo!()
    }
}