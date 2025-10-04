use chrono::Date;

pub enum Value {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Date(chrono::NaiveDate),
    Null
}
/*pub struct KV {
    pub(crate) key: String,
    pub(crate) value: Value
}

pub struct Record {
    columns: Vec<KV>
}*/

/*pub fn create(kvs: Vec<KV>) -> Record {
    Record {
        columns: kvs
    }
}*/

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