use crate::parser::entities::Record;
use crate::parser::error::ParserError;

pub trait Parse {
    fn parse(&self, path: &str) -> Result<Box<dyn Iterator<Item = Record>>, ParserError>;
}