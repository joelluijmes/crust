use std::fs;
use std::io;

use crate::location::Location;
use crate::token::{Token, TokenKind};

#[derive(Debug)]
pub struct Lexer {
    /// Path to the source code
    filepath: String,
    /// Source code of the file
    pub source: Vec<u8>,
    /// Current global location in the source
    cur: usize,
    /// Location of the current beginning of line
    bol: usize,
    /// Current row in the source
    row: usize,
}

#[derive(Debug)]
pub struct LexerError {
    /// Kind of the error
    pub kind: LexerErrorKind,
    /// Current location of the lexer where error happened
    pub loc: Location,
}

#[derive(Debug)]
pub enum LexerErrorKind {
    /// Unexpected unreachable state (or probably more likely unimplemented)
    Unreachable,
    /// Lexer reached the end of file
    UnexpectedEof,
}

impl Lexer {
    /// Creates a lexer from `filepath`
    pub fn from_file(filepath: &String) -> io::Result<Self> {
        let source = fs::read_to_string(filepath)?.into_bytes();

        Ok(Lexer {
            filepath: filepath.to_owned(),
            source,
            cur: 0,
            bol: 0,
            row: 0,
        })
    }

    /// Returns true when current position is at the end of source
    fn is_empty(&self) -> bool {
        self.cur >= self.source.len()
    }

    /// Returns true when current position is before end of source
    fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    /// Returns the current char in source, without advancing
    fn peek(&self) -> u8 {
        self.source[self.cur]
    }

    /// Advances lexer into the next position, while updating `bol` and `row`
    fn chop_char(&mut self) {
        if self.is_empty() {
            return;
        }

        let x = self.peek();
        self.cur += 1;

        if x == b'\n' {
            self.bol = self.cur;
            self.row += 1;
        }
    }

    /// Advances lexer until no whitespace is found
    fn trim_left(&mut self) {
        while self.is_not_empty() && self.peek().is_ascii_whitespace() {
            self.chop_char();
        }
    }

    /// Advances lexer until the next line
    fn drop_line(&mut self) {
        while self.is_not_empty() && self.peek() != b'\n' {
            self.chop_char();
        }
    }

    /// Returns the next token
    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        self.trim_left();

        // We don't support the preprocessor
        while self.is_not_empty() && self.peek() == b'#' {
            self.drop_line();
            self.trim_left();
        }

        if self.is_empty() {
            return Err(LexerError {
                kind: LexerErrorKind::UnexpectedEof,
                loc: self.loc(),
            });
        }

        // Save start location of the token
        let start_loc = self.loc();
        let start_cur = self.cur;

        // Parsing of an identifier
        if self.peek().is_ascii_alphabetic() {
            while self.is_not_empty() && self.peek().is_ascii_alphabetic() {
                self.chop_char();
            }

            let value = self.source[start_cur..self.cur].to_vec();
            let kind = self.tokenize_identifier(&value);

            return Ok(Token {
                kind,
                loc: start_loc,
                value,
            });
        }

        // Parsing of literals
        if let Some(kind) = match self.peek() {
            b'{' => Some(TokenKind::OpenCurly),
            b'}' => Some(TokenKind::CloseCurly),
            b'(' => Some(TokenKind::OpenParen),
            b')' => Some(TokenKind::CloseParen),
            b';' => Some(TokenKind::Semicolon),
            _ => None,
        } {
            self.chop_char();
            let value = self.source[start_cur..self.cur].to_vec();

            return Ok(Token {
                kind,
                loc: start_loc,
                value,
            });
        }

        // Parsing of string
        if self.peek() == b'\"' {
            self.chop_char();
            while self.is_not_empty() && self.peek() != b'\"' {
                self.chop_char();
            }
            self.chop_char();

            let value = self.source[start_cur..self.cur].to_vec();

            return Ok(Token {
                kind: TokenKind::String,
                loc: start_loc,
                value,
            });
        }

        // Parsing of number
        if self.peek().is_ascii_digit() {
            while self.is_not_empty() && self.peek().is_ascii_digit() {
                self.chop_char();
            }

            let value = self.source[start_cur..self.cur].to_vec();

            return Ok(Token {
                kind: TokenKind::Number,
                loc: start_loc,
                value,
            });
        }

        Err(LexerError {
            kind: LexerErrorKind::Unreachable,
            loc: self.loc(),
        })
    }

    /// Tries to parse the Identifier to a Keyword
    fn tokenize_identifier(&self, identifier: &Vec<u8>) -> TokenKind {
        match identifier.as_slice() {
            b"return" => TokenKind::KwReturn,
            b"int" => TokenKind::KwInt,
            _ => TokenKind::Identifier,
        }
    }

    /// Returns the curren); location of the lexer
    pub fn loc(&self) -> Location {
        Location {
            filepath: self.filepath.clone(),
            row: self.row,
            col: self.cur - self.bol,
        }
    }
}
