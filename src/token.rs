#[derive(Debug, PartialEq)]
pub enum Token {
    EOL(TokenDetails),
    EOF,
    EXCLAMATION(TokenDetails),
    COLON(char),
    PLUS(char),
    MINUS(char),
    STAR(char),
    NUMBER(Vec<char>),
    IDENTIFIER(Vec<char>),
    STRING(Vec<char>),
    ILLEGAL(Vec<char>),
}

#[derive(Debug, PartialEq)]
pub struct TokenDetails {
    pub row: usize,
    pub col: usize,
    pub literal: Vec<char>,
}

impl Token {
    fn length(&self) -> Option<usize> {
        match self {
            Token::EOL(details) => Some(details.literal.len()),
            _ => None,
        }
    }
}
