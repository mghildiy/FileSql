use rcsv_parser::parser::entities::csv_parser::CsvParser;
use rsql_executor::core::dataframe::DataFrame;
use rsql_executor::core::executor::Executor;
use rsql_parser::ast::constructs::{FromClause, SelectItem, SelectStatement, Statement, Value};

#[test]
fn fetch_all_data() {
    let csv_parser = CsvParser{};
    let mut executor = Executor {
        parser: Box::from(csv_parser)
    };
    let select_statement = Statement::Select(SelectStatement {
        columns: vec![SelectItem::Column("First Name".to_string()),
        ],
        from: Some(FromClause {
            source: "tests/test_data/test-data.csv".to_string(),
        }),
        where_clause: None,
        group_by: None,
        order_by: None,
    });
    let response  = executor.execute(select_statement);
    match response {
        Ok(DataFrame { columns, rows }) => {
            assert_eq!(columns.len(), 3);
            assert_eq!(columns[0], "First Name".to_string());
            assert_eq!(columns[1], "Last Name".to_string());
            assert_eq!(columns[3], "Gender".to_string());
            assert_eq!(rows.len(), 2);
            assert_eq!(rows[0].values.len(), 3);
            assert_eq!(rows[0].values[0], Value::String("Sheila".to_string()));
            assert_eq!(rows[0].values[1], Value::String("Mahoney".to_string()));
            assert_eq!(rows[0].values[2], Value::String("Female".to_string()));
            assert_eq!(rows[1].values.len(), 3);
            assert_eq!(rows[1].values[0], Value::String("Ram".to_string()));
            assert_eq!(rows[1].values[1], Value::String("Sharma".to_string()));
            assert_eq!(rows[1].values[2], Value::String("Male".to_string()));
        },
        Err(_) => panic!("Expected valid result, got error")
    }
}