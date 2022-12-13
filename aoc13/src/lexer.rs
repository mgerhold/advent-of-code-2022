use std::{error::Error, fmt::Display};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Token {
    LeftBracket,
    RightBracket,
    Comma,
    Integer(u64),
    EndOfInput,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Token::Integer(value) = self {
            f.write_fmt(format_args!("{value}"))
        } else {
            f.write_fmt(format_args!(
                "{}",
                match self {
                    Token::LeftBracket => "[",
                    Token::RightBracket => "]",
                    Token::Comma => ",",
                    Token::EndOfInput => "END_OF_INPUT",
                    _ => unreachable!(),
                }
            ))
        }
    }
}

pub(crate) struct Lexer<'a> {
    pub(crate) input: &'a [u8],
    pub(crate) index: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        Self { input, index: 0 }
    }

    pub fn current(&self) -> u8 {
        self.input[self.index]
    }

    pub fn peek(&self) -> Option<u8> {
        if self.index + 1 >= self.input.len() {
            None
        } else {
            Some(self.input[self.index + 1])
        }
    }

    pub fn advance(&mut self) {
        self.index += 1;
    }

    pub fn end_of_input(&self) -> bool {
        self.index >= self.input.len()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum LexerError {
    UnexpectedInput(u8),
    IntegerOutOfRange,
}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("lexer error: {:?}", self))
    }
}

impl Error for LexerError {}

pub(crate) fn tokenize(input: &str) -> Result<Vec<Token>, LexerError> {
    // we are only interested in ASCII chars
    assert!(input.is_ascii());
    let mut lexer = Lexer::new(input.as_bytes());
    let mut result = Vec::new();
    while !lexer.end_of_input() {
        match lexer.current() {
            b'[' => result.push(Token::LeftBracket),
            b']' => result.push(Token::RightBracket),
            b',' => result.push(Token::Comma),
            b'\n' => {
                // ignore newlines
            }
            c => {
                if !c.is_ascii_digit() {
                    return Err(LexerError::UnexpectedInput(c));
                }
                let mut to_parse = Vec::new();
                loop {
                    to_parse.push(lexer.current());
                    let next = lexer.peek();
                    if next.is_none() || !next.unwrap().is_ascii_digit() {
                        break;
                    }
                    lexer.advance();
                }
                let value = std::str::from_utf8(&to_parse)
                    .expect("str must only contain ASCII digits")
                    .parse::<u64>()
                    .map_err(|_| LexerError::IntegerOutOfRange)?;
                result.push(Token::Integer(value))
            }
        };
        lexer.advance();
    }
    result.push(Token::EndOfInput);
    Ok(result)
}
