use rsql_parser::ast::constructs::{AggregateFunc, BinaryOperator, Expr, FromClause, OrderByItem, SelectItem, SelectStatement, Statement, Value};
use rsql_parser::ast::constructs::Expr::{BinaryOp, Column};
use rsql_parser::lexer::grammar::GrammarType;
use rsql_parser::lexer::keywords::KeywordType;
use rsql_parser::lexer::operators::OperatorType;
use rsql_parser::lexer::tokenizer::tokenize;
use rsql_parser::lexer::tokens::Token;
use rsql_parser::parser::parser::Parser;

#[test]
fn tokenize_test() {
    let mut sql = "select * from \"users.csv\";";
    let mut tokens = tokenize(sql);
    assert_eq!(tokens.len(), 5);
    assert_eq!(tokens[0], Token::Keyword(KeywordType::Select));
    assert_eq!(tokens[1], Token::Grammar(GrammarType::Asterisk));
    assert_eq!(tokens[2], Token::Keyword(KeywordType::From));
    assert_eq!(tokens[3], Token::StringLiteral("users.csv".to_string()));
    assert_eq!(tokens[4], Token::Grammar(GrammarType::Semicolon));
    // test parser
    let mut parser = Parser::new(tokens);
    let mut response = parser.parse().unwrap();
    match response {
        Statement::Select(statement) => {
            let columns = statement.columns;
            assert_eq!(columns.len(), 1);
            assert_eq!(columns[0], SelectItem::Wildcard);
            let from = statement.from;
            match from {
                Some(FromClause { source }) => {
                    assert_eq!(source, "users.csv");
                },
                None => panic!("Expected from clause")
            }
            assert_eq!(statement.group_by, None);
            assert_eq!(statement.order_by, None);
        }
        _ => panic!("SelectStatement expected")
    }

    sql = "SELECT name FROM 'users';";
    tokens = tokenize(sql);
    assert_eq!(tokens.len(), 5);
    assert_eq!(tokens[0], Token::Keyword(KeywordType::Select));
    assert_eq!(tokens[1], Token::Identifier("name".to_string()));
    assert_eq!(tokens[2], Token::Keyword(KeywordType::From));
    assert_eq!(tokens[3], Token::StringLiteral("users".to_string()));
    assert_eq!(tokens[4], Token::Grammar(GrammarType::Semicolon));
    // test parser
    parser = Parser::new(tokens);
    response = parser.parse().unwrap();
    match response {
        Statement::Select(statement) => {
            let columns = statement.columns;
            assert_eq!(columns.len(), 1);
            assert_eq!(columns[0], SelectItem::Column("name".to_string()));
            let from = statement.from;
            match from {
                Some(FromClause { source }) => {
                    assert_eq!(source, "users");
                },
                None => panic!("Expected from clause")
            }
            assert_eq!(statement.group_by, None);
            assert_eq!(statement.order_by, None);
        },
        _ => panic!("SelectStatement expected")
    }

    sql = "SELECT id, name, age FROM \"employees\";";
    tokens = tokenize(sql);
    assert_eq!(tokens.len(), 9);
    assert_eq!(tokens[0], Token::Keyword(KeywordType::Select));
    assert_eq!(tokens[1], Token::Identifier("id".to_string()));
    assert_eq!(tokens[2], Token::Grammar(GrammarType::Comma));
    assert_eq!(tokens[3], Token::Identifier("name".to_string()));
    assert_eq!(tokens[4], Token::Grammar(GrammarType::Comma));
    assert_eq!(tokens[5], Token::Identifier("age".to_string()));
    assert_eq!(tokens[6], Token::Keyword(KeywordType::From));
    assert_eq!(tokens[7], Token::StringLiteral("employees".to_string()));
    assert_eq!(tokens[8], Token::Grammar(GrammarType::Semicolon));
    // test parser
    parser = Parser::new(tokens);
    response = parser.parse().unwrap();
    match response {
        Statement::Select(SelectStatement {
              columns   ,
              from, where_clause,
              group_by,
              order_by
        } ) => {
            assert_eq!(columns.len(), 3);
            assert_eq!(columns[0], SelectItem::Column("id".to_string()));
            assert_eq!(columns[1], SelectItem::Column("name".to_string()));
            assert_eq!(columns[2], SelectItem::Column("age".to_string()));
            match from {
                Some(FromClause { source }) => {
                    assert_eq!(source, "employees");
                },
                None => panic!("Expected from clause")
            }
            assert_eq!(group_by, None);
            assert_eq!(order_by, None);
        },
        _ => panic!("SelectStatement expected")
    }


    sql = "SELECT name, salary FROM 'employees' WHERE salary > 50000;";
    tokens = tokenize(sql);
    assert_eq!(tokens.len(), 11);
    assert_eq!(tokens[0], Token::Keyword(KeywordType::Select));
    assert_eq!(tokens[1], Token::Identifier("name".to_string()));
    assert_eq!(tokens[2], Token::Grammar(GrammarType::Comma));
    assert_eq!(tokens[3], Token::Identifier("salary".to_string()));
    assert_eq!(tokens[4], Token::Keyword(KeywordType::From));
    assert_eq!(tokens[5], Token::StringLiteral("employees".to_string()));
    assert_eq!(tokens[6], Token::Keyword(KeywordType::Where));
    assert_eq!(tokens[7], Token::Identifier("salary".to_string()));
    assert_eq!(tokens[8], Token::Operator(OperatorType::GreaterThan));
    assert_eq!(tokens[9], Token::Integer(50000));
    assert_eq!(tokens[10], Token::Grammar(GrammarType::Semicolon));
    parser = Parser::new(tokens);
    response = parser.parse().unwrap();
    match response {
        Statement::Select(SelectStatement {
                              columns   ,
                              from, where_clause,
                              group_by,
                              order_by
                          } ) => {
            assert_eq!(columns.len(), 2);
            assert_eq!(columns[0], SelectItem::Column("name".to_string()));
            assert_eq!(columns[1], SelectItem::Column("salary".to_string()));
            match from {
                Some(FromClause { source }) => {
                    assert_eq!(source, "employees");
                },
                None => panic!("Expected from clause")
            }
            let expected_where_expr = BinaryOp {
                left: Box::new(Expr::Column("salary".to_string())),
                operator: BinaryOperator::GreaterThan,
                right: Box::new(Expr::Literal(Value::Int(50000))),
            };
            assert_eq!(where_clause, Some(expected_where_expr));
            assert_eq!(group_by, None);
            assert_eq!(order_by, None);
        },
        _ => panic!("SelectStatement expected")
    }

    sql = "SELECT name FROM 'employees' WHERE department = 'HR' AND salary >= 40000;";
    tokens = tokenize(sql);
    assert_eq!(tokens.len(), 13);
    assert_eq!(tokens[0], Token::Keyword(KeywordType::Select));
    assert_eq!(tokens[1], Token::Identifier("name".to_string()));
    assert_eq!(tokens[2], Token::Keyword(KeywordType::From));
    assert_eq!(tokens[3], Token::StringLiteral("employees".to_string()));
    assert_eq!(tokens[4], Token::Keyword(KeywordType::Where));
    assert_eq!(tokens[5], Token::Identifier("department".to_string()));
    assert_eq!(tokens[6], Token::Operator(OperatorType::Equals));
    assert_eq!(tokens[7], Token::StringLiteral("HR".to_string()));
    assert_eq!(tokens[8], Token::Keyword(KeywordType::And));
    assert_eq!(tokens[9], Token::Identifier("salary".to_string()));
    assert_eq!(tokens[10], Token::Operator(OperatorType::GreaterThanOrEqual));
    assert_eq!(tokens[11], Token::Integer(40000));
    assert_eq!(tokens[12], Token::Grammar(GrammarType::Semicolon));
    parser = Parser::new(tokens);
    response = parser.parse().unwrap();
    match response {
        Statement::Select(SelectStatement {
                              columns,
                              from,
                              where_clause,
                              group_by,
                              order_by
                          } ) => {
            assert_eq!(columns.len(), 1);
            assert_eq!(columns[0], SelectItem::Column("name".to_string()));
            assert_eq!(from, Some(FromClause { source: "employees".to_string() }));
            match from {
                Some(FromClause { source }) => {
                    assert_eq!(source, "employees");
                },
                None => panic!("Expected from clause")
            }
            match where_clause {
                Some(Expr::BinaryOp {left, operator, right}) => {
                    let expected_left = Box::new(Expr::BinaryOp {
                        left: Box::new(Expr::Column(String::from("department"))),
                        operator: BinaryOperator::Equals,
                        right: Box::new(Expr::Literal(Value::String(String::from("HR")))),
                    });
                    assert_eq!(left, expected_left);
                },
                _ => panic!("Expected expression for where clause")
            }
            assert_eq!(group_by, None);
            assert_eq!(order_by, None);
        },
        _ => panic!("SelectStatement expected")
    }

    sql = "SELECT name, age FROM \"users\" ORDER BY age DESC;";
    tokens = tokenize(sql);
    assert_eq!(tokens.len(), 11);
    assert_eq!(tokens[0], Token::Keyword(KeywordType::Select));
    assert_eq!(tokens[1], Token::Identifier("name".to_string()));
    assert_eq!(tokens[2], Token::Grammar(GrammarType::Comma));
    assert_eq!(tokens[3], Token::Identifier("age".to_string()));
    assert_eq!(tokens[4], Token::Keyword(KeywordType::From));
    assert_eq!(tokens[5], Token::StringLiteral("users".to_string()));
    assert_eq!(tokens[6], Token::Keyword(KeywordType::Order));
    assert_eq!(tokens[7], Token::Keyword(KeywordType::By));
    assert_eq!(tokens[8], Token::Identifier("age".to_string()));
    assert_eq!(tokens[9], Token::Keyword(KeywordType::Desc));
    assert_eq!(tokens[10], Token::Grammar(GrammarType::Semicolon));
    parser = Parser::new(tokens);
    response = parser.parse().unwrap();
    match response {
        Statement::Select(SelectStatement {
                              columns,
                              from,
                              where_clause,
                              group_by,
                              order_by
                          }) => {
            assert_eq!(columns.len(), 2);
            assert_eq!(columns[0], SelectItem::Column("name".to_string()));
            assert_eq!(columns[1], SelectItem::Column("age".to_string()));
            match from {
                Some(FromClause { source }) => {
                    assert_eq!(source, "users");
                },
                None => panic!("Expected from clause")
            }
            assert_eq!(where_clause, None);
            assert_eq!(group_by, None);
            match order_by {
                Some(order_by_items) => {
                    assert_eq!(order_by_items.len(), 1);
                    let expected = OrderByItem {
                        expr: Column("age".to_string()),
                        asc: false,
                    };
                    assert_eq!(order_by_items[0], expected);
                },
                None => panic!("Expected order by item")
            }
        },
        _ => panic!("Expected select statement")
    }


    sql = "SELECT department, COUNT(*) FROM 'employees' GROUP BY department;";
    tokens = tokenize(sql);
    assert_eq!(tokens.len(), 13);
    assert_eq!(tokens[0], Token::Keyword(KeywordType::Select));
    assert_eq!(tokens[1], Token::Identifier("department".to_string()));
    assert_eq!(tokens[2], Token::Grammar(GrammarType::Comma));
    assert_eq!(tokens[3], Token::Keyword(KeywordType::Count));
    assert_eq!(tokens[4], Token::Grammar(GrammarType::OpenParen));
    assert_eq!(tokens[5], Token::Grammar(GrammarType::Asterisk));
    assert_eq!(tokens[6], Token::Grammar(GrammarType::CloseParen));
    assert_eq!(tokens[7], Token::Keyword(KeywordType::From));
    assert_eq!(tokens[8], Token::StringLiteral("employees".to_string()));
    assert_eq!(tokens[9], Token::Keyword(KeywordType::Group));
    assert_eq!(tokens[10], Token::Keyword(KeywordType::By));
    assert_eq!(tokens[11], Token::Identifier("department".to_string()));
    assert_eq!(tokens[12], Token::Grammar(GrammarType::Semicolon));
    parser = Parser::new(tokens);
    response = parser.parse().unwrap();
    match response {
        Statement::Select(SelectStatement {
                              columns,
                              from,
                              where_clause,
                              group_by,
                              order_by
                          }) => {
            assert_eq!(columns.len(), 2);
            assert_eq!(columns[0], SelectItem::Column("department".to_string()));
            assert_eq!(columns[1], SelectItem::Aggregate{
                func: AggregateFunc::Count,
                expr: None
            });
            match from {
                Some(FromClause { source }) => {
                    assert_eq!(source, "employees");
                },
                None => panic!("Expected from clause")
            }
            assert_eq!(where_clause, None);
            assert_eq!(order_by, None);
            match group_by {
                Some(group_by_items) => {
                    assert_eq!(group_by_items.len(), 1);
                    let expected = Expr::Column("department".to_string());
                    assert_eq!(group_by_items[0], expected);
                },
                None => panic!("Expected order by item")
            }
        },
        _ => panic!("Expected select statement")
    }

    sql = "SELECT department, name, COUNT(*) FROM 'employees' GROUP BY department, name;";
    tokens = tokenize(sql);
    assert_eq!(tokens.len(), 17);
    assert_eq!(tokens[0], Token::Keyword(KeywordType::Select));
    assert_eq!(tokens[1], Token::Identifier("department".to_string()));
    assert_eq!(tokens[2], Token::Grammar(GrammarType::Comma));
    assert_eq!(tokens[3], Token::Identifier("name".to_string()));
    assert_eq!(tokens[4], Token::Grammar(GrammarType::Comma));
    assert_eq!(tokens[5], Token::Keyword(KeywordType::Count));
    assert_eq!(tokens[6], Token::Grammar(GrammarType::OpenParen));
    assert_eq!(tokens[7], Token::Grammar(GrammarType::Asterisk));
    assert_eq!(tokens[8], Token::Grammar(GrammarType::CloseParen));
    assert_eq!(tokens[9], Token::Keyword(KeywordType::From));
    assert_eq!(tokens[10], Token::StringLiteral("employees".to_string()));
    assert_eq!(tokens[11], Token::Keyword(KeywordType::Group));
    assert_eq!(tokens[12], Token::Keyword(KeywordType::By));
    assert_eq!(tokens[13], Token::Identifier("department".to_string()));
    assert_eq!(tokens[14], Token::Grammar(GrammarType::Comma));
    assert_eq!(tokens[15], Token::Identifier("name".to_string()));
    assert_eq!(tokens[16], Token::Grammar(GrammarType::Semicolon));
    parser = Parser::new(tokens);
    response = parser.parse().unwrap();
    match response {
        Statement::Select(SelectStatement {
                              columns,
                              from,
                              where_clause,
                              group_by,
                              order_by
                          }) => {
            assert_eq!(columns.len(), 3);
            assert_eq!(columns[0], SelectItem::Column("department".to_string()));
            assert_eq!(columns[1], SelectItem::Column("name".to_string()));
            assert_eq!(columns[2], SelectItem::Aggregate{
                func: AggregateFunc::Count,
                expr: None
            });
            match from {
                Some(FromClause { source }) => {
                    assert_eq!(source, "employees");
                },
                None => panic!("Expected from clause")
            }
            assert_eq!(where_clause, None);
            assert_eq!(order_by, None);
            match group_by {
                Some(group_by_items) => {
                    assert_eq!(group_by_items.len(), 2);
                    let expected = Expr::Column("department".to_string());
                    assert_eq!(group_by_items[0], expected);
                    let expected = Expr::Column("name".to_string());
                    assert_eq!(group_by_items[1], expected);
                },
                None => panic!("Expected order by item")
            }
        },
        _ => panic!("Expected select statement")
    }


    sql = "SELECT name FROM users WHERE (age > 18 AND city = 'Delhi') OR city = 'Mumbai';";
    tokens = tokenize(sql);
    assert_eq!(tokens.len(), 19);
    assert_eq!(tokens[0], Token::Keyword(KeywordType::Select));
    assert_eq!(tokens[1], Token::Identifier("name".to_string()));
    assert_eq!(tokens[2], Token::Keyword(KeywordType::From));
    assert_eq!(tokens[3], Token::Identifier("users".to_string()));
    assert_eq!(tokens[4], Token::Keyword(KeywordType::Where));
    assert_eq!(tokens[5], Token::Grammar(GrammarType::OpenParen));
    assert_eq!(tokens[6], Token::Identifier("age".to_string()));
    assert_eq!(tokens[7], Token::Operator(OperatorType::GreaterThan));
    assert_eq!(tokens[8], Token::Integer(18));
    assert_eq!(tokens[9], Token::Keyword(KeywordType::And));
    assert_eq!(tokens[10], Token::Identifier("city".to_string()));
    assert_eq!(tokens[11], Token::Operator(OperatorType::Equals));
    assert_eq!(tokens[12], Token::StringLiteral("Delhi".to_string()));
    assert_eq!(tokens[13], Token::Grammar(GrammarType::CloseParen));
    assert_eq!(tokens[14], Token::Keyword(KeywordType::Or));
    assert_eq!(tokens[15], Token::Identifier("city".to_string()));
    assert_eq!(tokens[16], Token::Operator(OperatorType::Equals));
    assert_eq!(tokens[17], Token::StringLiteral("Mumbai".to_string()));
    assert_eq!(tokens[18], Token::Grammar(GrammarType::Semicolon));

    sql = "SELECT name FROM users WHERE city = 'New York';";
    tokens = tokenize(sql);
    assert_eq!(tokens.len(), 9);
    assert_eq!(tokens[0], Token::Keyword(KeywordType::Select));
    assert_eq!(tokens[1], Token::Identifier("name".to_string()));
    assert_eq!(tokens[2], Token::Keyword(KeywordType::From));
    assert_eq!(tokens[3], Token::Identifier("users".to_string()));
    assert_eq!(tokens[4], Token::Keyword(KeywordType::Where));
    assert_eq!(tokens[5], Token::Identifier("city".to_string()));
    assert_eq!(tokens[6], Token::Operator(OperatorType::Equals));
    assert_eq!(tokens[7], Token::StringLiteral("New York".to_string()));
    assert_eq!(tokens[8], Token::Grammar(GrammarType::Semicolon));

    sql = "SELECT name, price FROM products WHERE price < 99.99;";
    tokens = tokenize(sql);
    assert_eq!(tokens.len(), 11);
    assert_eq!(tokens[0], Token::Keyword(KeywordType::Select));
    assert_eq!(tokens[1], Token::Identifier("name".to_string()));
    assert_eq!(tokens[2], Token::Grammar(GrammarType::Comma));
    assert_eq!(tokens[3], Token::Identifier("price".to_string()));
    assert_eq!(tokens[4], Token::Keyword(KeywordType::From));
    assert_eq!(tokens[5], Token::Identifier("products".to_string()));
    assert_eq!(tokens[6], Token::Keyword(KeywordType::Where));
    assert_eq!(tokens[7], Token::Identifier("price".to_string()));
    assert_eq!(tokens[8], Token::Operator(OperatorType::SmallerThan));
    assert_eq!(tokens[9], Token::Float(99.99));
    assert_eq!(tokens[10], Token::Grammar(GrammarType::Semicolon));

    sql = "SELECT department, AVG(salary) FROM employees WHERE hire_date >= '2020-01-01' \
                        GROUP BY department ORDER BY AVG(salary) DESC;";
    tokens = tokenize(sql);
    assert_eq!(tokens.len(), 24);
    assert_eq!(tokens[0], Token::Keyword(KeywordType::Select));
    assert_eq!(tokens[1], Token::Identifier("department".to_string()));
    assert_eq!(tokens[2], Token::Grammar(GrammarType::Comma));
    assert_eq!(tokens[3], Token::Keyword(KeywordType::Avg));
    assert_eq!(tokens[4], Token::Grammar(GrammarType::OpenParen));
    assert_eq!(tokens[5], Token::Identifier("salary".to_string()));
    assert_eq!(tokens[6], Token::Grammar(GrammarType::CloseParen));
    assert_eq!(tokens[7], Token::Keyword(KeywordType::From));
    assert_eq!(tokens[8], Token::Identifier("employees".to_string()));
    assert_eq!(tokens[9], Token::Keyword(KeywordType::Where));
    assert_eq!(tokens[10], Token::Identifier("hire_date".to_string()));
    assert_eq!(tokens[11], Token::Operator(OperatorType::GreaterThanOrEqual));
    assert_eq!(tokens[12], Token::StringLiteral("2020-01-01".to_string()));
    assert_eq!(tokens[13], Token::Keyword(KeywordType::Group));
    assert_eq!(tokens[14], Token::Keyword(KeywordType::By));
    assert_eq!(tokens[15], Token::Identifier("department".to_string()));
    assert_eq!(tokens[16], Token::Keyword(KeywordType::Order));
    assert_eq!(tokens[17], Token::Keyword(KeywordType::By));
    assert_eq!(tokens[18], Token::Keyword(KeywordType::Avg));
    assert_eq!(tokens[19], Token::Grammar(GrammarType::OpenParen));
    assert_eq!(tokens[20], Token::Identifier("salary".to_string()));
    assert_eq!(tokens[21], Token::Grammar(GrammarType::CloseParen));
    assert_eq!(tokens[22], Token::Keyword(KeywordType::Desc));
    assert_eq!(tokens[23], Token::Grammar(GrammarType::Semicolon));

    sql = "SELECT name FROM users WHERE is_adult = TRUE;";
    tokens = tokenize(sql);
    assert_eq!(tokens.len(), 9);
    assert_eq!(tokens[0], Token::Keyword(KeywordType::Select));
    assert_eq!(tokens[1], Token::Identifier("name".to_string()));
    assert_eq!(tokens[2], Token::Keyword(KeywordType::From));
    assert_eq!(tokens[3], Token::Identifier("users".to_string()));
    assert_eq!(tokens[4], Token::Keyword(KeywordType::Where));
    assert_eq!(tokens[5], Token::Identifier("is_adult".to_string()));
    assert_eq!(tokens[6], Token::Operator(OperatorType::Equals));
    assert_eq!(tokens[7], Token::Keyword(KeywordType::True));
    assert_eq!(tokens[8], Token::Grammar(GrammarType::Semicolon));
}