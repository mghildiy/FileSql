use chrono::Date;

pub enum Value {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Date(chrono::NaiveDate),
}
pub struct KV {
    pub(crate) key: &str,
    pub(crate) value: Value
}

pub struct Record {
    columns: Vec<KV>
}

pub fn create(kvs: Vec<KV>) -> Record {
    Record {
        columns: kvs
    }
}

pub struct Table {
    columns: Vec<String>,
    records: Vec<Record>
}