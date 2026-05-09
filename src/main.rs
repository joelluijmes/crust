use std::fmt;
use std::fs;
use std::io;

#[derive(Debug)]
enum TokenKind {
    Name,
    OpenParen,
    CloseParen,
    OpenCurly,
    CloseCurly,
    Semicolon,
    Number,
    String,
}

struct Token {
    /// Kind of the token
    kind: TokenKind,
    /// Location of the token
    loc: Location,
    /// Value of the token
    value: Vec<u8>,
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

struct Location {
    /// Path to the source code
    filepath: String,
    /// Current row in the file
    row: usize,
    /// Current column of the line
    col: usize,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}:{}", self.filepath, self.row + 1, self.col + 1)
    }
}

impl fmt::Debug for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

#[derive(Debug)]
struct Lexer {
    /// Path to the source code
    filepath: String,
    /// Source code of the file
    source: Vec<u8>,
    /// Current global location in the source
    cur: usize,
    /// Location of the current beginning of line
    bol: usize,
    /// Current row in the source
    row: usize,
}

#[derive(Debug)]
enum LexerError {
    UnexpectedChar,
}

impl Lexer {
    /// Creates a lexer from `filepath`
    fn from_file(filepath: &str) -> io::Result<Self> {
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
    fn next_token(&mut self) -> Result<Option<Token>, LexerError> {
        self.trim_left();

        // We don't support the preprocessor
        while self.is_not_empty() && self.peek() == b'#' {
            self.drop_line();
            self.trim_left();
        }

        if self.is_empty() {
            return Ok(None);
        }

        // Save start location of the token
        let start_loc = self.loc();
        let start_cur = self.cur;

        // Parsing of a name / identifier
        if self.peek().is_ascii_alphabetic() {
            while self.is_not_empty() && self.peek().is_ascii_alphabetic() {
                self.chop_char();
            }

            let value = self.source[start_cur..self.cur].to_vec();

            return Ok(Some(Token {
                kind: TokenKind::Name,
                loc: start_loc,
                value,
            }));
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

            return Ok(Some(Token {
                kind,
                loc: start_loc,
                value,
            }));
        }

        // Parsing of string
        if self.peek() == b'\"' {
            self.chop_char();
            while self.is_not_empty() && self.peek() != b'\"' {
                self.chop_char();
            }
            self.chop_char();

            let value = self.source[start_cur..self.cur].to_vec();

            return Ok(Some(Token {
                kind: TokenKind::String,
                loc: start_loc,
                value,
            }));
        }

        // Parsing of number
        if self.peek().is_ascii_digit() {
            while self.is_not_empty() && self.peek().is_ascii_digit() {
                self.chop_char();
            }

            let value = self.source[start_cur..self.cur].to_vec();

            return Ok(Some(Token {
                kind: TokenKind::Number,
                loc: start_loc,
                value,
            }));
        }

        Err(LexerError::UnexpectedChar)
    }

    /// Returns the current location of the lexer
    fn loc(&self) -> Location {
        Location {
            filepath: self.filepath.clone(),
            row: self.row,
            col: self.cur - self.bol,
        }
    }
}

fn main() {
    let filepath = "examples/hello.c";

    let mut lexer = Lexer::from_file(filepath).expect("Failed to create lexer");
    println!("{:#?}", String::from_utf8(lexer.source.to_vec()));
    println!("---------------");

    while let Some(token) = lexer.next_token().expect("Failed to get next token") {
        println!("{:#?}", token);
    }
}
