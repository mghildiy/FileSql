#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Operators {
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

pub fn is_operator(word: &str) -> Option<Operators> {
    match word.to_ascii_uppercase().as_str() {
        "=" => Some(Operators::Equals),
        ">" => Some(Operators::GreaterThan),
        "<" => Some(Operators::SmallerThan),
        ">=" => Some(Operators::GreaterThanOrEqual),
        "<=" => Some(Operators::SmallerThanOrEqual),
        "<>" => Some(Operators::NotEquals),
        "&&" => Some(Operators::And),
        "||" => Some(Operators::Or),
        "+" => Some(Operators::Add),
        "-" => Some(Operators::Subtract),
        "/" => Some(Operators::Divide),
        "*" => Some(Operators::Multiply),
        _ => None
    }
}