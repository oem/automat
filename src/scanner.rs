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

fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}

fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}

fn is_string_delimiter(ch: char) -> bool {
    ch == '"'
}

#[derive(Debug)]
pub struct Scanner {
    input: Vec<char>,
    pub position: usize,
    pub read_position: usize,
    pub ch: Option<char>,
}

impl Scanner {
    pub fn new(input: Vec<char>) -> Self {
        let mut s = Self {
            input,
            position: 0,
            read_position: 0,
            ch: None,
        };
        s.read_char();
        s
    }

    pub fn scan(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token();
            if token == Token::EOF {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }
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

    pub fn skip_whitespace(&mut self) {
        if let Some(ch) = self.ch {
            if ch == ' ' || ch == '\t' {
                self.read_char();
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        let read_identifier = |l: &mut Scanner| -> Vec<char> {
            let position = l.position;
            while l.position < l.input.len() && l.ch.is_some() && is_letter(l.ch.unwrap()) {
                l.read_char();
            }
            l.input[position..l.position].to_vec()
        };

        let read_number = |l: &mut Scanner| -> Vec<char> {
            let position = l.position;
            while l.position < l.input.len() && l.ch.is_some() && is_digit(l.ch.unwrap()) {
                l.read_char();
            }
            l.input[position..l.position].to_vec()
        };

        let read_string = |l: &mut Scanner| -> Vec<char> {
            let position = l.position;
            while l.position < l.input.len()
                && l.ch.is_some()
                && !is_string_delimiter(l.ch.unwrap())
            {
                l.read_char();
            }
            l.input[position..l.position].to_vec()
        };

        self.skip_whitespace();

        let token_type: Token;
        if let Some(ch) = self.ch {
            token_type = match ch {
                t @ ':' => Token::COLON(t),
                t @ '!' => Token::EXCLAMATION(t),
                '\n' | '\r' => Token::EOL,
                '"' => {
                    self.read_char();
                    let str: Vec<char> = read_string(self);
                    Token::STRING(str)
                }
                'A'..='Z' | 'a'..='z' => {
                    let ident: Vec<char> = read_identifier(self);
                    return Token::IDENTIFIER(ident); // we don't want to call read_char after he
                                                     // match again, so we return here already
                }
                '0'..='9' => {
                    let num: Vec<char> = read_number(self);
                    return Token::NUMBER(num); // same here
                }
                t @ _ => Token::ILLEGAL(vec![t]),
            };
        } else {
            token_type = Token::EOF;
        }

        self.read_char();
        token_type
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
            Token::COLON(':'),
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
            Token::EXCLAMATION('!'),
            Token::EOF,
        ];
        let mut l = Scanner::new(input);
        let actual = l.scan();
        assert_eq!(actual, expected);
    }
}
