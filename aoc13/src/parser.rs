use std::{error::Error, fmt::Display};

use crate::lexer::Token;

#[derive(Debug, Clone)]
pub(crate) enum Expression {
    Integer(u64),
    List(Vec<Expression>),
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Integer(value) => f.write_fmt(format_args!("{value}")),
            Expression::List(sub_expressions) => {
                let sub_expressions: Vec<_> = sub_expressions
                    .iter()
                    .map(|expression| format!("{expression}"))
                    .collect();
                f.write_fmt(format_args!("[{}]", sub_expressions.join(",")))
            }
        }
    }
}

pub(crate) struct Parser {
    pub(crate) tokens: Vec<Token>,
    pub(crate) index: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ParserError {
    UnexpectedToken(Token),
    ExpectedToken(Token),
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("parser error: {:?}", self))
    }
}

impl Error for ParserError {}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, index: 0 }
    }

    pub fn current(&self) -> &Token {
        &self.tokens[self.index]
    }

    pub fn advance(&mut self) {
        self.index += 1
    }

    pub fn end_of_input(&self) -> bool {
        matches!(self.current(), Token::EndOfInput) || self.index >= self.tokens.len()
    }

    pub(crate) fn expression(&mut self) -> Result<Expression, ParserError> {
        match *self.current() {
            Token::LeftBracket => self.list(),
            Token::Integer(value) => {
                self.advance();
                Ok(Expression::Integer(value))
            }
            _ => {
                return Err(ParserError::UnexpectedToken(*self.current()));
            }
        }
    }

    pub fn list(&mut self) -> Result<Expression, ParserError> {
        assert!(matches!(self.current(), Token::LeftBracket));
        self.advance();
        if matches!(self.current(), Token::RightBracket) {
            // empty list
            self.advance();
            return Ok(Expression::List(Vec::new()));
        }
        let mut sub_expressions = Vec::new();
        sub_expressions.push(self.expression()?);
        while matches!(self.current(), Token::Comma) {
            self.advance();
            sub_expressions.push(self.expression()?);
        }
        if !matches!(self.current(), Token::RightBracket) {
            Err(ParserError::ExpectedToken(Token::RightBracket))
        } else {
            self.advance();
            Ok(Expression::List(sub_expressions))
        }
    }
}

pub(crate) fn parse(tokens: Vec<Token>) -> Result<Vec<Expression>, ParserError> {
    let mut parser = Parser::new(tokens);
    let mut result = Vec::new();
    while !parser.end_of_input() {
        result.push(parser.list()?);
    }
    Ok(result)
}
