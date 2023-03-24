#[derive(Debug, PartialEq)]
pub enum Token {
    EOL(TokenDetails),
    EOF,
    Ignored,
    Exclamation(TokenDetails),
    Colon(TokenDetails),
    Plus(TokenDetails),
    Minus(TokenDetails),
    Star(TokenDetails),
    Percentage(TokenDetails),
    Number(TokenDetails),
    Identifier(TokenDetails),
    String(TokenDetails),
    Illegal(TokenDetails),
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
