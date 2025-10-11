#[derive(Debug, PartialEq, Eq, Clone)]
pub enum GrammarType {
    Comma,
    Semicolon,
    Asterisk,
    OpenParen,
    CloseParen,
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