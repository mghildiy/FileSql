use std::fmt::Display;
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

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let token = match self {
            Token::Keyword(kw) => format!("Keyword {}", kw),
            Token::Identifier(id) => format!("Identifier {}", id),
            Token::Integer(i) => format!("Integer {}", i),
            Token::Float(f) => format!("Float {}", f),
            Token::StringLiteral(s) => format!("String literal {}", s),
            Token::Grammar(gr) => format!("Grammar marker {}", gr),
            Token::Operator(o) => format!("Operator {}", o)
        };

        write!(f, "{}", token)
    }
}

