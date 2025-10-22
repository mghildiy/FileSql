use crate::ast::constructs::SelectItem::{Column, Wildcard};
use crate::ast::constructs::{AggregateFunc, Expr, SelectItem, SelectStatement, Statement};
use crate::lexer::grammar::GrammarType;
use crate::lexer::keywords::KeywordType;
use crate::lexer::operators::OperatorType;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::Token::{Grammar, Identifier, Keyword};
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
        match self.peek()? {
            Keyword(KeywordType::Select) => {
                return self.parse_select().map(Statement::Select)
            },
            _ => return Err(ParserError{
                message: "Expected SELECT keyword".to_string(),
                position: self.position,
            }),
        }
    }

    fn parse_select(&mut self) -> Result<SelectStatement, ParserError> {
        self.expect_keyword(KeywordType::Select)?;
        let select_items = self.parse_columns()?;
        let select_statement = SelectStatement {
            columns: select_items,
            from: None,
            where_clause: None,
            group_by: vec![],
            order_by: vec![],
        };

        Ok(select_statement)
    }

    fn parse_columns(&mut self) -> Result<Vec<SelectItem>, ParserError> {
        let mut selected_items = Vec::new();
        loop {
            let item = self.parse_select_item()?;
            selected_items.push(item);

            match self.peek()? {
                Token::Grammar(GrammarType::Comma) => {
                    self.advance()?;  // consume comma, continue loop
                }
                Token::Keyword(KeywordType::From) => {
                    break;  // columns done
                }
                other => {
                    return Err(ParserError {
                        message: format!("Expected comma or FROM, {} found", other),
                        position: self.position
                    });
                }
            }
        }

        Ok(selected_items)
    }

    fn parse_select_item(&mut self) -> Result<SelectItem, ParserError> {
        match self.peek()? {
            // wildcard: SELECT *
            Grammar(GrammarType::Asterisk) => {
                self.advance()?;
                Ok(Wildcard)
            }

            // aggregate functions: COUNT, AVG, SUM, MIN, MAX
            Keyword(kw) if self.is_aggregate_keyword(kw) => {
                self.parse_aggregate()
            }

            // Regular column: SELECT name
            Identifier(_) => {
                let name = self.expect_identifier()?;
                Ok(Column(name))
            }

            other => Err(ParserError {
                message: format!("Expected column, *, or aggregate, found {:?}", other),
                position: self.position
            })
        }
    }

    fn is_aggregate_keyword(&self, kw: &KeywordType) -> bool {
        matches!(kw,
            KeywordType::Count |
            KeywordType::Sum |
            KeywordType::Avg |
            KeywordType::Min |
            KeywordType::Max
        )
    }

    fn parse_aggregate(&mut self) -> Result<SelectItem, ParserError> {
        // get the aggregate function type
        let func = match self.advance()? {
            Keyword(KeywordType::Count) => AggregateFunc::Count,
            Keyword(KeywordType::Avg) => AggregateFunc::Avg,
            Keyword(KeywordType::Sum) => AggregateFunc::Sum,
            Keyword(KeywordType::Min) => AggregateFunc::Min,
            Keyword(KeywordType::Max) => AggregateFunc::Max,
            other => return Err(ParserError {
                message: format!("Expected one of COUNT, SUM, AVG, MIN, MAX, found {}", other),
                position: self.position
            })
        };

        // expect opening parenthesis
        self.expect_grammar(GrammarType::OpenParen)?;

        // parse between parentheses: either * or expression
        let expr = match self.peek()? {
            Grammar(GrammarType::Asterisk) => {
                self.advance()?;
                None  // COUNT(*) case
            }
            _ => {
                // for now, just parse simple column, later parse full expressions
                let col = self.expect_identifier()?;
                Some(Box::new(Expr::Column(col)))
            }
        };

        // Expect closing parenthesis
        self.expect_grammar(GrammarType::CloseParen)?;

        Ok(SelectItem::Aggregate { func, expr })
    }

    fn peek(&self) -> Result<&Token, ParserError> {
        // TODO: in sql parser module, ensure that white space is ignored so that a token is captured
        // in entirety ..SELECT Avg  (salary)      FROM employees.csv
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