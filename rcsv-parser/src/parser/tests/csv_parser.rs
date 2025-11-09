use crate::parser::entities::entities::Value;
use crate::parser::parser_factory::get_parser;
use crate::parser::entities::file_types::FileType;

#[test]
fn small_structured_file() {
    let mut csv_parser = get_parser(&FileType::CSV).ok().expect("CSV parser expected");
    let test_data = "src/parser/tests/test_data/people-01.csv";

    let response = csv_parser.parse(test_data, Some("%Y-%m-%d".to_string()));

    match response {
        Ok(result) => {
            let mut iter = result.0;
            let header = result.1;

            assert!(header.is_some());
            assert_eq!(header.unwrap().len(), 9);
            let cols =iter.next().unwrap().columns;
            assert_eq!(cols.len(), 9);
            let mut expected: Vec<Value> = Vec::new();
            expected.push(Value::Int(1));
            expected.push(Value::String("8717bbf45cCDbEe".to_string()));
            expected.push(Value::String("Shelia".to_string()));
            expected.push(Value::String("Mahoney".to_string()));
            expected.push(Value::String("Male".to_string()));
            expected.push(Value::String("pwarner@example.org".to_string()));
            expected.push(Value::String("857.139.8239".to_string()));
            expected.push(Value::Date(chrono::NaiveDate::from_ymd_opt(2014, 01, 27).unwrap()));
            expected.push(Value::String("Probation officer".to_string()));
            check_for_equality(&cols, &expected);
        },
        Err(e) => panic!("Test failed with parser error: {}", e.message)
    }
}

fn check_for_equality(actual: &Vec<Value>, expected: &Vec<Value>) {
    assert_eq!(actual.len(), expected.len());
    for (actual, expected) in actual.iter().zip(expected.iter()) {
        assert_eq!(actual, expected);
    }
}

#[test]
fn empty_file() {
    let mut csv_parser = get_parser(&FileType::CSV).ok().expect("CSV parser expected");
    let test_data = "src/parser/tests/test_data/empty.csv";

    let response = csv_parser.parse(test_data, None);
    assert!(response.is_err());
    assert_eq!(response.err().unwrap().message, "File is empty")
}

#[test]
fn only_header_file() {
    let mut csv_parser = get_parser(&FileType::CSV).ok().expect("CSV parser expected");
    let test_data = "src/parser/tests/test_data/only_header.csv";

    let response = csv_parser.parse(test_data, None);

    match response {
        Ok(result) => {
            let mut iter = result.0;
            let header = result.1;
            assert!(header.is_some());
            assert_eq!(header.unwrap().len(), 3);
            assert!(iter.next().is_none())
        },
        Err(e) => panic!("Test failed with parser error: {}", e.message)
    }
}

#[test]
fn file_with_missing_column() {
    let mut csv_parser = get_parser(&FileType::CSV).ok().expect("CSV parser expected");
    let test_data = "src/parser/tests/test_data/file_with_missing_column.csv";

    let response = csv_parser.parse(test_data, None);

    match response {
        Ok(result) => {
            let mut iter = result.0;
            let header = result.1;
            assert!(header.is_some());
            assert_eq!(header.unwrap().len(), 9);
            assert!(iter.next().is_some());
            let second_record =iter.next().unwrap().columns;
            assert_eq!(second_record.len(), 9);
            let maybe_null = second_record.get(2).unwrap();
            assert_eq!(Value::Null, *maybe_null);
        },
        Err(e) => panic!("Test failed with parser error: {}", e.message)
    }
}