#[derive(Debug, PartialEq)]
pub enum Value {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Date(chrono::NaiveDate),
    Null
}

pub fn create(values: Vec<Value>) -> Record {
    Record {
        columns: values
    }
}

pub struct Record {
    pub(crate) columns: Vec<Value>
}

pub struct Table {
    columns: Vec<String>,
    records: Vec<Record>
}