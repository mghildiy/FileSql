use crate::core::dataframe::{DataFrame, Row};
use crate::core::errors::ExecutorError;
use rcsv_parser::parser::entities::entities::Value as CsvValue;
use rcsv_parser::parser::entities::parser::Parse;
use rcsv_parser::parser::entities::Record;
use rsql_parser::ast::constructs::{SelectStatement, Statement, Value};

struct Executor {
    parser: dyn Parse
}

impl Executor {
    pub fn execute(&mut self, statement: Statement) -> Result<DataFrame, ExecutorError> {
        match statement {
            Statement::Select(select) => self.execute_select(select),
            _ => Err(ExecutorError {
                message: "Unsupported statement.".to_string(),
            })
        }
    }

    fn execute_select(&mut self, select: SelectStatement) -> Result<DataFrame, ExecutorError> {
        match select.from {
            Some(from) => {
                match self.parser.parse(&*from.source, None) {
                    Ok((mut record_iterator, header)) => {
                        Ok(DataFrame {
                            columns: header.unwrap(),
                            rows: consume(Box::new(record_iterator))
                        })
                    },
                    Err(pe) => return Err(ExecutorError {message: pe.message})
                }
            },
            None => return Err(ExecutorError {
                message: "Data source is required.".to_string()
            })
        }
    }
}

fn consume(mut iterator: Box<dyn Iterator<Item=Record>>) -> Vec<Row> {
    let mut rows: Vec<Row> = Vec::new();
    while let Some(record) = iterator.next() {
        let df_vals = record.columns
            .into_iter()
            .map(|val| rawparser_val_to_datafame_val(val))
            .collect();
        let row  = Row {values: df_vals};
        rows.push(row)
    }
    return rows
}

fn rawparser_val_to_datafame_val(val: CsvValue) -> Value {
    match val {
        CsvValue::String(str) => {
            Value::String(str)
        },
        CsvValue::Float(float) => {
            Value::Float(float)
        },
        CsvValue::Int(int) => {
            Value::Int(int)
        },
        CsvValue::Bool(bool) => {
            Value::Bool(bool)
        },
        CsvValue::Null => {
            Value::Null
        },
        CsvValue::Date(date) => {
            Value::Date(date)
        }
    }
}