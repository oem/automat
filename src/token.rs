#[derive(Debug, PartialEq)]
pub enum Token {
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
