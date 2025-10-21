use std::fmt::Display;

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

impl Display for KeywordType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keyword = match self {
            KeywordType::Select => "SELECT",
            KeywordType::From => "FROM",
            KeywordType::Where => "WHERE",
            KeywordType::Group => "GROUP",
            KeywordType::Order => "ORDER",
            KeywordType::By => "BY",
            KeywordType::And => "AND",
            KeywordType::Or => "OR",
            KeywordType::Asc => "ASC",
            KeywordType::Desc => "DESC",
            KeywordType::Count => "COUNT",
            KeywordType::Avg => "AVG",
            KeywordType::Min => "MIN",
            KeywordType::Max => "MAX",
        };

        write!(f, "{}", keyword)
    }
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