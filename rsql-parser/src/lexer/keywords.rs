#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeywordType {
    Select,
    From,
    Where,
    Group,
    Order,
    By,
    And,
    Or,
    Asc,
    Desc,
    Count,
    Avg,
    Min,
    Max
}

pub fn is_keyword(word: &str) -> Option<KeywordType> {
    match word.to_ascii_uppercase().as_str() {
        "SELECT" => Some(KeywordType::Select),
        "FROM" => Some(KeywordType::From),
        "WHERE" => Some(KeywordType::Where),
        "ORDER" => Some(KeywordType::Order),
        "GROUP" => Some(KeywordType::Group),
        "BY" => Some(KeywordType::By),
        "AND" => Some(KeywordType::And),
        "OR" => Some(KeywordType::Or),
        "ASC" => Some(KeywordType::Asc),
        "DESC" => Some(KeywordType::Desc),
        "COUNT" => Some(KeywordType::Count),
        "AVG" => Some(KeywordType::Avg),
        "MIN" => Some(KeywordType::Min),
        "MAX" => Some(KeywordType::Max),
        _ => None,
    }
}