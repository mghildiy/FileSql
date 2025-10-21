use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum GrammarType {
    Comma,
    Semicolon,
    Asterisk,
    OpenParen,
    CloseParen,
}

impl Display for GrammarType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let grammar_type = match self {
            GrammarType::Comma => ",",
            GrammarType::Semicolon => ";",
            GrammarType::Asterisk => "*",
            GrammarType::OpenParen => "(",
            GrammarType::CloseParen => ")",
        };

        write!(f, "{}", grammar_type)
    }
}

pub fn is_grammar_type(word: &str) -> Option<GrammarType> {
    match word.to_ascii_uppercase().as_str() {
        "," => Some(GrammarType::Comma),
        "*" => Some(GrammarType::Asterisk),
        "(" => Some(GrammarType::OpenParen),
        ")" => Some(GrammarType::CloseParen),
        ";" => Some(GrammarType::Semicolon),
        _ => None,
    }
}