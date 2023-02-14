#[derive(Debug, PartialEq)]
pub enum TokenType {
    EOL,
    EOF,
    EXCLAMATION,
    COLON,
    PLUS,
    MINUS,
    STAR,
    NUMBER(Vec<char>),
    IDENTIFIER(Vec<char>),
    STRING(Vec<char>),
    ILLEGAL(Vec<char>),
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
}
