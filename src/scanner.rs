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

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = Some(self.input[self.read_position])
        }
        self.position = self.read_position;
        self.read_position = self.read_position + 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assignment() {
        let input = "1:x".chars().collect();
        let expected = vec![
            Token::NUMBER(vec!['1']),
            Token::COLON,
            Token::IDENTIFIER(vec!['x']),
            Token::EOF,
        ];
        let mut l = Scanner::new(input);
        let actual = l.scan();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_enum() {
        let input = "12!".chars().collect();
        let expected = vec![
            Token::NUMBER(vec!['1', '2']),
            Token::EXCLAMATION,
            Token::EOF,
        ];
        let mut l = Scanner::new(input);
        let actual = l.scan();
        assert_eq!(actual, expected);
    }
}
