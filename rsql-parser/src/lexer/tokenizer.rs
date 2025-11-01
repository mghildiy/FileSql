use crate::lexer::errors::LexerError;
use crate::lexer::grammar::is_grammar_type;
use crate::lexer::keywords::is_keyword;
use crate::lexer::operators::is_operator;
use crate::lexer::tokenizer::LEXER_STATE::{IN_IDENTIFIER, START};
use crate::lexer::tokens::Token;
use crate::lexer::tokens::Token::{Float, Grammar, Identifier, Integer, Keyword, Operator, StringLiteral};

#[derive(PartialEq, Eq)]
enum LEXER_STATE {
    START,
    IN_IDENTIFIER,
    IN_NUMBER,
    IN_STRING(char),
    IN_OPERATOR
}

const WHITESPACE_TERMINATORS: &[char] = &[' ', '\t', '\n'];
const PUNCTUATION_TERMINATORS: &[char] = &[',', ';', '(', ')', '*'];

const OPERATORS: &[char] = &['=', '!', '<', '>'];

pub fn tokenize(sql: &str) -> Vec<Token> {
    let response = state_based_tokenize(sql);
    match response {
        Ok(tokens) => tokens,
        Err(error) => {
            println!("Error while parsing lexer: {}", error.message);
            panic!("{:?}", "Error while parsing sql");
        }
    }
}

pub fn state_based_tokenize(sql: &str) -> Result<Vec<Token>, LexerError> {
    let mut words: Vec<String> = Vec::new();
    let trimmed = sql.trim();
    let mut chars = trimmed.chars();
    let mut state = START;
    let mut current_word = String::new();
    while let Some(current_char) = chars.next() {
        match &mut state {
            START => {
                let response = handle_start_state(&current_char, &mut current_word, &mut state);
                match response {
                    Ok(possible_word) => {
                        if let Some(word) = possible_word {
                            words.push(word.to_string());
                        }
                    }
                    Err(e) => return Err(e)
                }
            }
            IN_IDENTIFIER => {
                let response = handle_in_identifier_state(&current_char, &mut current_word, &mut state);
                match response {
                    Ok(possible_word) => {
                        if let Some(terminator) = possible_word {
                            words.push(current_word.clone());
                            current_word.clear();
                            if PUNCTUATION_TERMINATORS.contains(&terminator.chars().next().unwrap()) {
                                words.push(terminator);
                            }
                        }
                    }
                    Err(e) => return Err(e)
                }
            }
            LEXER_STATE::IN_STRING(_) => {
                let response = handle_in_string_state(&current_char, &mut current_word, &mut state);
                match response {
                    Ok(possible_word) => {
                        if let Some(word) = possible_word {
                            words.push(word);
                        }
                    }
                    Err(e) => return Err(e)
                }
            }
            LEXER_STATE::IN_NUMBER => {
                let response = handle_in_number_state(&current_char, &mut current_word, &mut state);
                match response {
                    Ok(possible_word) => {
                        if let Some(terminator) = possible_word {
                            words.push(current_word.clone());
                            current_word.clear();
                            if PUNCTUATION_TERMINATORS.contains(&terminator.chars().next().unwrap()) {
                                words.push(terminator);
                            }
                        }
                    }
                    Err(e) => return Err(e)
                }
            }
            LEXER_STATE::IN_OPERATOR => {
                let response = handle_in_operator_state(&current_char, &mut current_word, &mut state);
                match response {
                    Ok(possible_word) => {
                        if let Some(terminator) = possible_word {
                            words.push(current_word.clone());
                            current_word.clear();
                            if PUNCTUATION_TERMINATORS.contains(&terminator.chars().next().unwrap()) {
                                words.push(terminator);
                            }
                        }
                    },
                    Err(e) => return Err(e)
                }
            }
            _ => return Err(LexerError{message: "Error".to_string()})
        }
    }

    if current_word.len() > 0 {
        words.push(current_word);
    }
    Ok(assign_tokens(&words))
}

fn handle_in_operator_state(
    ch: &char,
    current_word: &mut String,
    state: &mut LEXER_STATE
) -> Result<Option<String>, LexerError> {
    if is_operator_char(*ch) {
        current_word.push(*ch);
        Ok(None)
    } else if WHITESPACE_TERMINATORS.contains(ch)
        || PUNCTUATION_TERMINATORS.contains(ch)
        || ch.is_alphanumeric()
    {
        *state = START;
        Ok(Some(ch.to_string()))
    } else {
        Err(LexerError {
            message: "Unexpected character in operator".to_string(),
        })
    }
}

fn handle_in_number_state(
    ch: &char,
    current_word: &mut String,
    state: &mut LEXER_STATE
) -> Result<Option<String>, LexerError> {
    if(ch.is_digit(10)) {
        current_word.push(*ch);
        Ok(None)
    } else if *ch == '.' {
        if(current_word.contains(".")) {
            return Err(LexerError { message: "More than one decimals in number".to_string() })
        }
        current_word.push(*ch);
        return Ok(None)
    } else if WHITESPACE_TERMINATORS.contains(ch) || PUNCTUATION_TERMINATORS.contains(ch) {
        *state = START;
        return Ok(Some(ch.to_string()));
    } else {
        return Err(LexerError {message: "Unexpected character".to_string()});
    }
}


// TODO YET TO HANDLE Escaped quotes inside strings("\"Hi\" All")
fn handle_in_string_state(
    ch: &char,
    current_word: &mut String,
    state: &mut LEXER_STATE
) -> Result<Option<String>, LexerError> {
    if let LEXER_STATE::IN_STRING(quote_char) = *state {
        if *ch != quote_char {
            current_word.push(*ch);
            Ok(None)
        } else {
            current_word.push(*ch);
            let word = current_word.clone();
            current_word.clear();
            *state = LEXER_STATE::START;
            Ok(Some(word))
        }
    } else {
        return Err(LexerError {message: "Not IN_STRING state".to_string()})
    }
}

fn handle_in_identifier_state(ch: &char, current_word: &mut String, state: &mut LEXER_STATE)
    -> Result<Option<String>, LexerError> {
    if ch.is_alphanumeric() || *ch == '_' {
        current_word.push(*ch);
        return Ok(None);
    } else if WHITESPACE_TERMINATORS.contains(ch) || PUNCTUATION_TERMINATORS.contains(ch) {
        *state = START;
        return Ok(Some(ch.to_string()));
    } else {
        return Err(LexerError {message: "Unexpected character".to_string()});
    }
}

fn handle_start_state(ch: &char, current_word: &mut String, state: &mut LEXER_STATE)
    -> Result<Option<String>, LexerError> {
    if(WHITESPACE_TERMINATORS.contains(ch)) {
        return Ok(None)
    } else if(ch.is_alphabetic() || *ch == '_') {
        *state = IN_IDENTIFIER;
        current_word.push(*ch);
        return Ok(None)
    } else if ch.is_digit(10) {
        *state = LEXER_STATE::IN_NUMBER;
        current_word.push(*ch);
        return Ok(None)
    } else if *ch == '"' || *ch == '\'' {
        *state = LEXER_STATE::IN_STRING(*ch);
        current_word.push(*ch);
        return Ok(None)
    } else if is_operator_char(*ch) {
        *state = LEXER_STATE::IN_OPERATOR;
        current_word.push(*ch);
        return Ok(None)
    } else if PUNCTUATION_TERMINATORS.contains(ch) {
        return Ok(Some(ch.to_string()))
    } else  {
        return Err(LexerError {message: "Unexpected character".to_string()})
    }
}

// TODO add more operators as and when needed
fn is_operator_char(c: char) -> bool {
    //matches!(c, '=' | '!' | '<' | '>')
    OPERATORS.contains(&c)
}

fn split_based_tokenize(sql: &str) -> Vec<Token> {
    let trimmed = sql.trim();
    let mut chars = trimmed.chars();
    if trimmed.ends_with(";") {
        chars.next_back();
    }
    let mut current_word = String::new();
    let mut words: Vec<String> = Vec::new();
    while let Some(current) = chars.next() {
        if(current.is_whitespace()) {
            // add current word only if it has some characters, as in case of multiple whitespaces
            // we may have already added thr word ot list
            if(current_word.len() > 0) {
                words.push(current_word.clone());
                current_word.clear();
            }
            continue
        }
        current_word.push(current);
    }
    if(current_word.len() > 0) {
        words.push(current_word);
    }

    assign_tokens(&words)
}

fn assign_tokens(words: &Vec<String>) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    for word in words {
        if let Some(keyword_type) = is_keyword(word) {
            tokens.push(Keyword(keyword_type))
        } else if let Some(grammar_type) = is_grammar_type(word) {
            tokens.push(Grammar(grammar_type));
        } else if let Some(operator) = is_operator(word) {
            tokens.push(Operator(operator));
        } else if word.parse::<i64>().is_ok() {
            tokens.push(Integer(word.parse::<i64>().unwrap()));
        } else if word.parse::<f64>().is_ok() {
            tokens.push(Float(word.parse::<f64>().unwrap()));
        } else if word.starts_with("\"") && word.ends_with("\"") {
            let trimmed = &word[1..word.len() - 1];
            tokens.push(StringLiteral(trimmed.parse().unwrap()));
        } else if word.starts_with("\'") && word.ends_with("\'") {
            let trimmed = &word[1..word.len() - 1];
            tokens.push(StringLiteral(trimmed.parse().unwrap()));
        } else {
            tokens.push(Identifier(word.to_string()));
        }
    }

    tokens
}