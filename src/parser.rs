use crate::lexer::{Lexer, LexerError, LexerErrorKind};
use crate::location::Location;
use crate::token::{Token, TokenKind};

#[derive(Debug)]
pub struct Parser {
    /// Lexer pointing to the source code
    lexer: Lexer,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct ParserError {
    pub kind: ParserErrorKind,
    pub loc: Location,
    pub token: Option<Token>,
}

impl ParserError {
    pub fn with_token(kind: ParserErrorKind, token: Token) -> Self {
        let loc = token.loc.clone();
        ParserError {
            kind,
            loc,
            token: Some(token),
        }
    }

    pub fn with_location(kind: ParserErrorKind, loc: Location) -> Self {
        ParserError {
            kind,
            loc,
            token: None,
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ParserErrorKind {
    /// `expect_token` didn't found one of the expected tokens.
    UnexpectedToken { expected_kinds: Vec<TokenKind> },
    /// Received a token from the lexer we can't parse (yet).
    UnparsableToken,
    /// Received a type we can't parse (yet).
    NotImplementedType(String),
    /// Failed to convert the text into a number.
    FailedToParseNumber,
    /// Lexer raised an error during parsing.
    LexerError(LexerErrorKind),
}

#[derive(Debug)]
pub enum Type {
    Int,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Statement {
    Funcall { name: String, args: Vec<String> },
    Declare { name: String },
    Initialize { name: String, value: Expr },
    Assign { name: String, value: Expr },
    Return { return_value: Expr },
}

#[derive(Debug)]
pub enum Expr {
    LitInt(i32),
    Identifier(String),
}

impl From<LexerError> for ParserError {
    fn from(v: LexerError) -> Self {
        Self::with_location(ParserErrorKind::LexerError(v.kind), v.loc)
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

    pub fn lexer(&mut self) -> &mut Lexer {
        &mut self.lexer
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
        let token = self.expect_token(TokenKind::KwInt)?;

        if token.value != b"int" {
            return Err(ParserError::with_token(
                ParserErrorKind::NotImplementedType(format!(
                    "Type '{}' not supported",
                    String::from_utf8_lossy(&token.value)
                )),
                token,
            ));
        }

        Ok(Type::Int)
    }

    fn parse_function_name(&mut self) -> Result<String, ParserError> {
        Ok(
            String::from_utf8(self.expect_token(TokenKind::Identifier)?.value)
                .expect("Failed to convert ASCII"),
        )
    }

    fn parse_args(&mut self) -> Result<Vec<String>, ParserError> {
        let mut args = Vec::<String>::new();
        loop {
            let token = self.expect_token_one_of(&[
                TokenKind::String,
                TokenKind::Number,
                TokenKind::CloseParen,
            ])?;

            match token.kind {
                TokenKind::CloseParen => break,
                _ => args.push(String::from_utf8(token.value).expect("Failed to convert ASCII")),
            }
        }

        Ok(args)
    }

    fn parse_block(&mut self) -> Result<Block, ParserError> {
        self.expect_token(TokenKind::OpenCurly)?;

        let mut body = Block::new();

        loop {
            let token = self.next_token()?;

            match token.kind {
                TokenKind::CloseCurly => return Ok(body),

                _ => {
                    // TODO: check if statement or func call
                    let statement = self.parse_statement(token)?;
                    body.push(statement);
                }
            }
        }
    }

    fn parse_statement(&mut self, token: Token) -> Result<Statement, ParserError> {
        match token.kind {
            TokenKind::KwReturn => {
                let return_value = self.expect_token(TokenKind::Number)?.value_as_int();
                self.expect_token(TokenKind::Semicolon)?;

                Ok(Statement::Return {
                    return_value: Expr::LitInt(return_value),
                })
            }

            TokenKind::KwInt => {
                let variable_name = self.expect_token(TokenKind::Identifier)?;

                // Either it is an initialization or a declaration only
                match self
                    .expect_token_one_of(&[TokenKind::Eq, TokenKind::Semicolon])?
                    .kind
                {
                    TokenKind::Eq => {
                        self.parse_variable_assignment(variable_name.value_as_string())
                    }

                    TokenKind::Semicolon => Ok(Statement::Declare {
                        name: variable_name.value_as_string(),
                    }),

                    _ => unreachable!(),
                }
            }

            TokenKind::Identifier => {
                let variable_name = token.value_as_string();

                // Either it is an assignment or a funccall
                match self
                    .expect_token_one_of(&[TokenKind::Eq, TokenKind::OpenParen])?
                    .kind
                {
                    TokenKind::Eq => self.parse_variable_assignment(variable_name),

                    TokenKind::OpenParen => {
                        // TODO: actually check what the name is instead supporting just printf
                        if variable_name == "printf" {
                            let args = self.parse_args()?;

                            self.expect_token(TokenKind::Semicolon)?;

                            return Ok(Statement::Funcall {
                                name: variable_name,
                                args,
                            });
                        }

                        todo!()
                    }
                    _ => unreachable!(),
                }
            }

            _ => {
                return Err(ParserError::with_token(
                    ParserErrorKind::UnparsableToken,
                    token.clone(),
                ));
            }
        }
    }

    fn parse_variable_assignment(
        &mut self,
        variable_name: String,
    ) -> Result<Statement, ParserError> {
        // TODO: refactor into parsing proper expressions
        let token = self.expect_token_one_of(&[TokenKind::Number, TokenKind::Identifier])?;

        match token.kind {
            // E.g. int x = <int-literal>
            TokenKind::Number => {
                self.expect_token(TokenKind::Semicolon)?;

                Ok(Statement::Initialize {
                    name: variable_name,
                    value: Expr::LitInt(token.value_as_int()),
                })
            }

            // E.g. int x = var;
            TokenKind::Identifier => {
                self.expect_token(TokenKind::Semicolon)?;

                Ok(Statement::Assign {
                    name: variable_name,
                    value: Expr::Identifier(token.value_as_string()),
                })
            }

            _ => unreachable!(),
        }
    }

    /// Returns the next token from the lexer
    fn next_token(&mut self) -> Result<Token, ParserError> {
        Ok(self.lexer.next_token()?)
    }

    /// Gathers the next token, compare if it matches the passed `kind`, if so return that.
    /// Otherwise an error is returned.
    fn expect_token(&mut self, kind: TokenKind) -> Result<Token, ParserError> {
        self.expect_token_one_of(&[kind])
    }

    /// Gathers the next token, compare if it matches one of the passed `kinds`, if so return that.
    /// Otherwise an error is returned.
    fn expect_token_one_of(&mut self, kinds: &[TokenKind]) -> Result<Token, ParserError> {
        match self.next_token()? {
            token if kinds.contains(&token.kind) => Ok(token),
            token => todo!(
                "Unexpected token, got {:?} but expected {:?}\n  at: {:?}",
                token.kind,
                kinds,
                token.loc
            ), // token => Err(ParserError::with_token(
               //     ParserErrorKind::UnexpectedToken {
               //         expected_kinds: kinds.to_vec(),
               //     },
               //     token,
               // )),
        }
    }
}
