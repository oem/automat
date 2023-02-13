#[derive(Debug, PartialEq)]
pub enum Token {
    EOL,
    EOF,
    EXCLAMATION,
    COLON,
    PLUS,
    MINUS,
    NUMBER(Vec<char>),
    IDENTIFIER(Vec<char>),
}
