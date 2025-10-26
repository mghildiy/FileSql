use crate::ast::constructs::SelectItem::{Column, Wildcard};
use crate::ast::constructs::{AggregateFunc, BinaryOperator, Expr, FromClause, SelectItem, SelectStatement, Statement, Value};
use crate::ast::constructs::Expr::Literal;
use crate::lexer::grammar::GrammarType;
use crate::lexer::keywords::KeywordType;
use crate::lexer::operators::OperatorType;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::Token::{Float, Grammar, Identifier, Integer, Keyword, StringLiteral};
use crate::parser::errors::ParserError;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {tokens, position: 0}
    }

    pub fn parse(&mut self) -> Result<Statement, ParserError> {
        self.parse_statement()
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
        let select_statement = SelectStatement {
            columns: self.parse_columns()?,
            from: self.parse_from()?,
            where_clause: self.parse_where()?,
            group_by: None,
            order_by: None,
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
            // TODO: but this means even non-count aggregate functions may be paired with *,
            // so need to handle case by case
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

    fn parse_from(&mut self) -> Result<Option<FromClause>, ParserError> {
        self.expect_keyword(KeywordType::From)?;
        let source = self.expect_string_literal()
            .map_err(|e| ParserError {
                message: format!("Expected string literal after FROM: {}", e.message),
                position: e.position
            })?;
        Ok(Some(FromClause { source }))
    }

    fn parse_where(&mut self) -> Result<Option<Expr>, ParserError> {
        if !matches!(self.peek()?, Token::Keyword(KeywordType::Where)) {
            return Ok(None);
        }

        self.expect_keyword(KeywordType::Where)?;
        let expr = self.parse_or_expression()?;
        Ok(Some(expr))
    }
    fn parse_or_expression(&mut self) -> Result<Expr, ParserError> {
        let mut left = self.parse_and_expression()?;

        while matches!(self.peek()?, Token::Keyword(KeywordType::Or)) {
            self.advance()?;
            let right = self.parse_and_expression()?;

            left = Expr::BinaryOp {
                left: Box::new(left),
                operator: BinaryOperator::Or,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_and_expression(&mut self) -> Result<Expr, ParserError> {
        let mut left = self.parse_comparison()?;

        while matches!(self.peek()?, Token::Keyword(KeywordType::And)) {
            self.advance()?;
            let right = self.parse_comparison()?;

            left = Expr::BinaryOp {
                left: Box::new(left),
                operator: BinaryOperator::And,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expr, ParserError> {
        // Left side
        let left = self.parse_primary()?;

        // Operator
        let operator = self.parse_comparison_operator()?;

        // Right side
        let right = self.parse_primary()?;

        Ok(Expr::BinaryOp {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }

    fn parse_comparison_operator(&mut self) -> Result<BinaryOperator, ParserError> {
        let token = self.advance()?;

        match token {
            Token::Operator(op) => {
                match op {
                    OperatorType::Equals => Ok(BinaryOperator::Equals),
                    OperatorType::NotEquals => Ok(BinaryOperator::NotEquals),
                    OperatorType::GreaterThan => Ok(BinaryOperator::GreaterThan),
                    OperatorType::SmallerThan => Ok(BinaryOperator::LessThan),
                    OperatorType::GreaterThanOrEqual => Ok(BinaryOperator::GreaterThanOrEquals),
                    OperatorType::SmallerThanOrEqual => Ok(BinaryOperator::LessThanOrEquals),
                    _ => Err(ParserError {
                        message: format!("Expected comparison operator, found {}", op),
                        position: self.position - 1
                    })
                }
            }
            other => Err(ParserError {
                message: format!("Expected operator, found {}", other),
                position: self.position - 1
            })
        }
    }

    fn parse_primary(&mut self) -> Result<Expr, ParserError> {
        match self.peek()? {
            Identifier(_) => {
                let identifier = self.expect_identifier()?;
                Ok(Expr::Column(identifier))
            },
            Integer(_) => {
                let value = self.expect_integer()?;
                Ok(Literal(Value::Int(value)))
            },
            Float(_) => {
                let value = self.expect_float()?;
                Ok(Literal(Value::Float(value)))
            },
            StringLiteral(_) => {
                let value = self.expect_string_literal()?;
                Ok(Literal(Value::String(value)))
            },
            Keyword(KeywordType::True) => {
                self.advance()?;
                Ok(Literal(Value::Bool(true)))
            },
            Keyword(KeywordType::False) => {
                self.advance()?;
                Ok(Literal(Value::Bool(false)))
            },
            _ => Err(ParserError {
                message: "Expected column or literal".to_string(),
                position: self.position
            })
        }
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
            Keyword(k) if k == &kw => {
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
            Token::Identifier(ident) => Ok(ident),
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