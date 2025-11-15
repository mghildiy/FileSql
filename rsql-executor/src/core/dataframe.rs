use rsql_parser::ast::constructs::Value;

pub struct DataFrame {
    pub columns: Vec<String>,
    pub rows: Vec<Row>
}

pub struct Row {
    pub values: Vec<Value>
}