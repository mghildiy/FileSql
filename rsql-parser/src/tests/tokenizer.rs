use crate::lexer::grammar::GrammarType;
use crate::lexer::keywords::KeywordType;
use crate::lexer::operators::OperatorType;
use crate::lexer::tokenizer::tokenize;
use crate::lexer::tokens::Token;

#[test]
fn tokenize_test() {
    let mut sql = "select * from users;";
    let mut tokens = tokenize(sql);
    assert_eq!(tokens.len(), 5);
    assert_eq!(tokens[0], Token::Keyword(KeywordType::Select));
    assert_eq!(tokens[1], Token::Grammar(GrammarType::Asterisk));
    assert_eq!(tokens[2], Token::Keyword(KeywordType::From));
    assert_eq!(tokens[3], Token::Identifier("users".to_string()));
    assert_eq!(tokens[4], Token::Grammar(GrammarType::Semicolon));

    sql = "SELECT name FROM users;";
    tokens = tokenize(sql);
    assert_eq!(tokens.len(), 5);
    assert_eq!(tokens[0], Token::Keyword(KeywordType::Select));
    assert_eq!(tokens[1], Token::Identifier("name".to_string()));
    assert_eq!(tokens[2], Token::Keyword(KeywordType::From));
    assert_eq!(tokens[3], Token::Identifier("users".to_string()));
    assert_eq!(tokens[4], Token::Grammar(GrammarType::Semicolon));

    sql = "SELECT id, name, age FROM employees;";
    tokens = tokenize(sql);
    assert_eq!(tokens.len(), 9);
    assert_eq!(tokens[0], Token::Keyword(KeywordType::Select));
    assert_eq!(tokens[1], Token::Identifier("id".to_string()));
    assert_eq!(tokens[2], Token::Grammar(GrammarType::Comma));
    assert_eq!(tokens[3], Token::Identifier("name".to_string()));
    assert_eq!(tokens[4], Token::Grammar(GrammarType::Comma));
    assert_eq!(tokens[5], Token::Identifier("age".to_string()));
    assert_eq!(tokens[6], Token::Keyword(KeywordType::From));
    assert_eq!(tokens[7], Token::Identifier("employees".to_string()));
    assert_eq!(tokens[8], Token::Grammar(GrammarType::Semicolon));

    sql = "SELECT name, salary FROM employees WHERE salary > 50000";
    tokens = tokenize(sql);
    assert_eq!(tokens.len(), 10);
    assert_eq!(tokens[0], Token::Keyword(KeywordType::Select));
    assert_eq!(tokens[1], Token::Identifier("name".to_string()));
    assert_eq!(tokens[2], Token::Grammar(GrammarType::Comma));
    assert_eq!(tokens[3], Token::Identifier("salary".to_string()));
    assert_eq!(tokens[4], Token::Keyword(KeywordType::From));
    assert_eq!(tokens[5], Token::Identifier("employees".to_string()));
    assert_eq!(tokens[6], Token::Keyword(KeywordType::Where));
    assert_eq!(tokens[7], Token::Identifier("salary".to_string()));
    assert_eq!(tokens[8], Token::Operator(OperatorType::GreaterThan));
    assert_eq!(tokens[9], Token::Integer(50000));

    sql = "SELECT name FROM employees WHERE department = 'HR' AND salary >= 40000;";
    tokens = tokenize(sql);
    assert_eq!(tokens.len(), 13);
    assert_eq!(tokens[0], Token::Keyword(KeywordType::Select));
    assert_eq!(tokens[1], Token::Identifier("name".to_string()));
    assert_eq!(tokens[2], Token::Keyword(KeywordType::From));
    assert_eq!(tokens[3], Token::Identifier("employees".to_string()));
    assert_eq!(tokens[4], Token::Keyword(KeywordType::Where));
    assert_eq!(tokens[5], Token::Identifier("department".to_string()));
    assert_eq!(tokens[6], Token::Operator(OperatorType::Equals));
    assert_eq!(tokens[7], Token::Identifier("HR".to_string()));
    assert_eq!(tokens[8], Token::Keyword(KeywordType::And));
    assert_eq!(tokens[9], Token::Identifier("salary".to_string()));
    assert_eq!(tokens[10], Token::Operator(OperatorType::GreaterThanOrEqual));
    assert_eq!(tokens[11], Token::Integer(40000));
    assert_eq!(tokens[12], Token::Grammar(GrammarType::Semicolon));

    sql = "SELECT name, age FROM users ORDER BY age DESC;";
    tokens = tokenize(sql);
    assert_eq!(tokens.len(), 11);
    assert_eq!(tokens[0], Token::Keyword(KeywordType::Select));
    assert_eq!(tokens[1], Token::Identifier("name".to_string()));
    assert_eq!(tokens[2], Token::Grammar(GrammarType::Comma));
    assert_eq!(tokens[3], Token::Identifier("age".to_string()));
    assert_eq!(tokens[4], Token::Keyword(KeywordType::From));
    assert_eq!(tokens[5], Token::Identifier("users".to_string()));
    assert_eq!(tokens[6], Token::Keyword(KeywordType::Order));
    assert_eq!(tokens[7], Token::Keyword(KeywordType::By));
    assert_eq!(tokens[8], Token::Identifier("age".to_string()));
    assert_eq!(tokens[9], Token::Keyword(KeywordType::Desc));
    assert_eq!(tokens[10], Token::Grammar(GrammarType::Semicolon));


    sql = "SELECT department, COUNT(*) FROM employees GROUP BY department;";
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
    assert_eq!(tokens[8], Token::Identifier("employees".to_string()));
    assert_eq!(tokens[9], Token::Keyword(KeywordType::Group));
    assert_eq!(tokens[10], Token::Keyword(KeywordType::By));
    assert_eq!(tokens[11], Token::Identifier("department".to_string()));
    assert_eq!(tokens[12], Token::Grammar(GrammarType::Semicolon));


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
    assert_eq!(tokens[12], Token::Identifier("Delhi".to_string()));
    assert_eq!(tokens[13], Token::Grammar(GrammarType::CloseParen));
    assert_eq!(tokens[14], Token::Keyword(KeywordType::Or));
    assert_eq!(tokens[15], Token::Identifier("city".to_string()));
    assert_eq!(tokens[16], Token::Operator(OperatorType::Equals));
    assert_eq!(tokens[17], Token::Identifier("Mumbai".to_string()));
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
    assert_eq!(tokens[7], Token::Identifier("New York".to_string()));
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
    assert_eq!(tokens[12], Token::Identifier("2020-01-01".to_string()));
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

}