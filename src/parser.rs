use crate::lexer::{Lexer, LexerError};
use crate::token::{Token, TokenKind};

#[derive(Debug)]
pub struct Parser {
    /// Lexer pointing to the source code
    lexer: Lexer,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ParserError {
    UnexpectedToken {
        expected_kinds: Vec<TokenKind>,
        received_token: Token,
    },
    UnexpectedEof,
    UnexpectedName,
    NotImplementedType(String),
    FailedToParseNumber,
    LexerError(LexerError),
}

#[derive(Debug)]
pub enum Type {
    Int,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Statement {
    Funcall,
    Return { return_value: i32 },
}

impl From<LexerError> for ParserError {
    fn from(v: LexerError) -> Self {
        Self::LexerError(v)
    }
}

type Block = Vec<Statement>;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Function {
    /// Name of the function
    pub name: String,
    /// Return type of the function
    pub return_type: Type,
    /// Implementation of the function
    pub body: Block,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Parser { lexer }
    }

    pub fn parse_program(&mut self) -> Result<Function, ParserError> {
        self.parse_function()
    }

    fn parse_function(&mut self) -> Result<Function, ParserError> {
        let return_type = self.parse_type()?;
        let name = self.parse_function_name()?;

        // TODO: parse args
        self.expect_token(TokenKind::OpenParen)?;
        self.expect_token(TokenKind::CloseParen)?;

        let body = self.parse_block()?;

        Ok(Function {
            name,
            return_type,
            body,
        })
    }

    fn parse_type(&mut self) -> Result<Type, ParserError> {
        let token = self.expect_token(TokenKind::Name)?;

        if token.value != b"int" {
            return Err(ParserError::NotImplementedType(format!(
                "Type '{}' not supported",
                String::from_utf8_lossy(&token.value)
            )));
        }

        Ok(Type::Int)
    }

    fn parse_function_name(&mut self) -> Result<String, ParserError> {
        Ok(String::from_utf8(self.expect_token(TokenKind::Name)?.value)
            .expect("Failed to convert ASCII"))
    }

    fn parse_block(&mut self) -> Result<Block, ParserError> {
        self.expect_token(TokenKind::OpenCurly)?;

        let mut body = Block::new();

        loop {
            let token = self.expect_token_one_of(&[TokenKind::Name, TokenKind::CloseCurly])?;
            match token.kind {
                TokenKind::CloseCurly => return Ok(body),
                TokenKind::Name => match token.value.as_slice() {
                    b"return" => {
                        let token_return_value = self.expect_token(TokenKind::Number)?;
                        let return_value = String::from_utf8(token_return_value.value)
                            .expect("Invalid ASCII characters")
                            .parse::<i32>()
                            .map_err(|_| ParserError::FailedToParseNumber)?;

                        self.expect_token(TokenKind::Semicolon)?;

                        body.push(Statement::Return { return_value })
                    }
                    _ => return Err(ParserError::UnexpectedName),
                },
                _ => unreachable!(),
            }
        }
    }

    /// Gathers the next token, compare if it matches the passed `kind`, if so return that.
    /// Otherwise an error is returned.
    fn expect_token(&mut self, kind: TokenKind) -> Result<Token, ParserError> {
        self.expect_token_one_of(&[kind])
    }

    /// Gathers the next token, compare if it matches one of the passed `kinds`, if so return that.
    /// Otherwise an error is returned.
    fn expect_token_one_of(&mut self, kinds: &[TokenKind]) -> Result<Token, ParserError> {
        match self.lexer.next_token()? {
            Some(token) if kinds.contains(&token.kind) => Ok(token),
            Some(token) => Err(ParserError::UnexpectedToken {
                expected_kinds: kinds.to_vec(),
                received_token: token,
            }),
            None => Err(ParserError::UnexpectedEof),
        }
    }
}
