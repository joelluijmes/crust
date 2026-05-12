use std::fmt;

use crate::location::Location;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Identifier,
    OpenParen,
    CloseParen,
    OpenCurly,
    CloseCurly,
    Semicolon,
    Number,
    String,

    // Keywords
    KwReturn,
    KwInt,

    // Operations
    OpAssign,
}

#[derive(Clone)]
pub struct Token {
    /// Kind of the token
    pub kind: TokenKind,
    /// Location of the token
    pub loc: Location,
    /// Value of the token
    pub value: Vec<u8>,
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Token {{ kind: {:?}, loc: {}, value: {:?} }}",
            self.kind,
            self.loc,
            String::from_utf8_lossy(&self.value),
        )
    }
}

impl Token {
    /// Converts the `value` to `String`
    pub fn value_as_string(self) -> String {
        String::from_utf8(self.value).expect("Invalid ASCII characters")
    }
}
