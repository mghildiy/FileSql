use rcsv_parser::parser::entities::parser::Parse;
use rsql_parser::ast::constructs::{SelectStatement, Statement};
use crate::core::dataframe::DataFrame;
use crate::core::errors::ExecutorError;

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
                self.parser.parse(&*from.source, None);
                todo!()
            },
            None => return Err(crate::core::errors::ExecutorError {
                message: "".to_string()
            })
        }
    }
}