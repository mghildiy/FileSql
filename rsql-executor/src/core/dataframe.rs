use rsql_parser::ast::constructs::Value;

pub struct DataFrame {
    columns: Vec<String>,
    rows: Vec<Row>
}

pub struct Row {
    values: Vec<Value>
}