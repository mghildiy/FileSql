use crate::parser::entities::record_iterator::RecordIterator;
use crate::parser::error::ParserError;
pub trait Parse {
    fn parse(&mut self, file_path: &str, date_format: Option<String>) -> Result<(RecordIterator, Option<Vec<String>>), ParserError>;
}