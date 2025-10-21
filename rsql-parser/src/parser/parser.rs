use crate::ast::constructs::Statement;
use crate::lexer::grammar::GrammarType;
use crate::lexer::keywords::KeywordType;
use crate::lexer::operators::OperatorType;
use crate::lexer::tokens::Token;
use crate::parser::errors::ParserError;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {tokens, position: 0}
    }

    pub fn parse(&self, tokens: Vec<Token>) -> Result<Statement, ParserError> {
        let mut parser = Parser { tokens, position: 0};
        parser.parse_statement()
    }

    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        todo!()
    }

    fn peek(&self) -> Result<&Token, ParserError> {
        self.tokens.get(self.position)
            .ok_or_else(
                || ParserError{message: "Unexpected end of tokens".to_string(), position: self.position }
            )
    }

    fn advance (&mut self) -> Result<Token, ParserError> {
        let token = self.peek()?.clone();
        self.position += 1;
        Ok(token)
    }

    fn expect_keyword(&mut self, kw: KeywordType) -> Result<(), ParserError> {
        let token = self.peek()?;
        match token {
            Token::Keyword(k) if k == &kw => {
                self.advance()?;
                Ok(())
            },
            _ => Err(ParserError{message: format!("Expected keyword '{}'", kw), position: self.position})
        }
    }

    fn expect_grammar(&mut self, grammar: GrammarType) -> Result<(), ParserError> {
        let token = self.peek()?;
        match token {
            Token::Grammar(g) if g == &grammar => {
                self.advance()?;
                Ok(())
            },
            _ => Err(
                ParserError{
                    message: format!("Expected grammar element '{}'", grammar),
                    position: self.position
                })
        }
    }

    fn expect_identifier(&mut self) -> Result<String, ParserError> {
        let token = self.advance()?;
        match token {
            Token::Identifier(ident) => Ok(ident.clone()),
            _ => Err(
                ParserError{
                    message: "Expected an identifier".to_string(),
                    position: self.position - 1
                })
        }
    }

    fn expect_string_literal(&mut self) -> Result<String, ParserError> {
        let token = self.advance()?;
        match token {
            Token::StringLiteral(literal) => Ok(literal.clone()),
            _ => Err(
                ParserError{
                    message: "Expected a string literal".to_string(),
                    position: self.position - 1
            })
        }
    }

    fn expect_integer(&mut self) -> Result<i64, ParserError> {
        let token = self.advance()?;
        match token {
            Token::Integer(number) => Ok(number),
            _ => Err(ParserError{message: "Expected an integer literal".to_string(), position: self.position - 1})
        }
    }

    fn expect_float(&mut self) -> Result<f64, ParserError> {
        let token = self.advance()?;
        match token {
            Token::Float(number) => Ok(number),
            _ => Err(ParserError{message: "Expected a float literal".to_string(), position: self.position - 1})
        }
    }

    fn expect_operator(&mut self, operator: OperatorType) -> Result<(), ParserError> {
        let token = self.peek()?;
        match token {
            Token::Operator(o) if o == &operator => {
                self.advance()?;
                Ok(())
            },
            _ => Err(ParserError{message: format!("Expected operator '{}'", operator), position: self.position})
        }
    }
}