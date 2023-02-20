#[derive(Debug, PartialEq)]
pub enum Token {
    EOL(TokenDetails),
    EOF,
    EXCLAMATION(TokenDetails),
    COLON(TokenDetails),
    PLUS(TokenDetails),
    MINUS(TokenDetails),
    STAR(TokenDetails),
    PERCENTAGE(TokenDetails),
    NUMBER(TokenDetails),
    IDENTIFIER(TokenDetails),
    STRING(TokenDetails),
    ILLEGAL(TokenDetails),
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
