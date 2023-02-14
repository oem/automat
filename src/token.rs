#[derive(Debug, PartialEq)]
pub enum TokenType {
    EOL,
    EOF,
    EXCLAMATION(char),
    COLON(char),
    PLUS(char),
    MINUS(char),
    STAR(char),
    NUMBER(Vec<char>),
    IDENTIFIER(Vec<char>),
    STRING(Vec<char>),
    ILLEGAL(Vec<char>),
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
}
