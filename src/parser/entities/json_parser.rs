use crate::parser::entities::parser::Parse;
use crate::parser::entities::Record;
use crate::parser::error::ParserError;

pub struct JSONParser {}

impl Parse for JSONParser {
    fn parse(&self, path: &str) -> Result<Box<dyn Iterator<Item=Record>>, ParserError> {
        todo!()
    }
}