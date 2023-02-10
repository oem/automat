use crate::token::Token;

#[derive(Debug)]
pub enum ScannerError {
    UnknownTokenError,
}

impl std::fmt::Display for ScannerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ScannerError {}

#[derive(Debug)]
pub struct Scanner {
    input: Vec<char>,
    pub position: usize,
    pub read_position: usize,
    pub ch: Option<char>,
}

impl Scanner {
    pub fn new(input: Vec<char>) -> Self {
        Self {
            input,
            position: 0,
            read_position: 0,
            ch: None,
        }
    }

    pub fn scan(&mut self) -> Vec<Token> {
        let tokens = Vec::new();
        tokens
    }
}

