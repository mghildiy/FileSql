use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum OperatorType {
    Equals,
    GreaterThan,
    SmallerThan,
    GreaterThanOrEqual,
    SmallerThanOrEqual,
    NotEquals,
    And,
    Or,
    Add,
    Subtract,
    Divide,
    Multiply
}

impl Display for OperatorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let operator = match self {
            OperatorType::Equals => "=",
            OperatorType::GreaterThan => ">",
            OperatorType::SmallerThan => "<",
            OperatorType::GreaterThanOrEqual => ">=",
            OperatorType::SmallerThanOrEqual => "<=",
            OperatorType::NotEquals => "!=",
            OperatorType::And => "AND",
            OperatorType::Or => "OR",
            OperatorType::Add => "+",
            OperatorType::Subtract => "-",
            OperatorType::Divide => "/",
            OperatorType::Multiply => "*"
        };
        
        write!(f, "{}", operator)
    }
}

pub fn is_operator(word: &str) -> Option<OperatorType> {
    match word.to_ascii_uppercase().as_str() {
        "=" => Some(OperatorType::Equals),
        ">" => Some(OperatorType::GreaterThan),
        "<" => Some(OperatorType::SmallerThan),
        ">=" => Some(OperatorType::GreaterThanOrEqual),
        "<=" => Some(OperatorType::SmallerThanOrEqual),
        "<>" => Some(OperatorType::NotEquals),
        "&&" => Some(OperatorType::And),
        "||" => Some(OperatorType::Or),
        "+" => Some(OperatorType::Add),
        "-" => Some(OperatorType::Subtract),
        "/" => Some(OperatorType::Divide),
        "*" => Some(OperatorType::Multiply),
        _ => None
    }
}