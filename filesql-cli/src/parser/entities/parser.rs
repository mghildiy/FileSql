use crate::parser::entities::Record;
use crate::parser::error::ParserError;

pub type RecordIterator = Box<dyn Iterator<Item = Record>>;
pub trait Parse {
    fn parse(&mut self, path: &str) -> Result<RecordIterator, ParserError>;
}