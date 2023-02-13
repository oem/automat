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
}
