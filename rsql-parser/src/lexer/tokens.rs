use crate::lexer::grammar::GrammarType;
use crate::lexer::keywords::KeywordType;
use crate::lexer::operators::OperatorType;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Keyword(KeywordType),
    Identifier(String),
    Integer(i64),
    Float(f64),
    StringLiteral(String),  // quoted strings
    Grammar(GrammarType),
    Operator(OperatorType),
}

