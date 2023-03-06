use crate::token::{Token, TokenDetails};

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

fn is_String_delimiter(ch: char) -> bool {
    ch == '"'
}

#[derive(Debug)]
pub struct Scanner {
    input: Vec<char>,
    pub position: usize,
    pub read_position: usize,
    pub ch: Option<char>,
    row: usize,
    col: usize,
}

impl Scanner {
    pub fn new(input: Vec<char>) -> Self {
        let mut s = Self {
            input,
            position: 0,
            read_position: 0,
            ch: None,
            row: 0,
            col: 0,
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
        self.col = self.col + 1;
    }

    pub fn next_token(&mut self) -> Token {
        let read_Identifier = |l: &mut Scanner| -> Vec<char> {
            let position = l.position;
            while l.position < l.input.len() && l.ch.is_some() && is_letter(l.ch.unwrap()) {
                l.read_char();
            }
            l.input[position..l.position].to_vec()
        };

        let read_Number = |l: &mut Scanner| -> Vec<char> {
            let position = l.position;
            while l.position < l.input.len() && l.ch.is_some() && is_digit(l.ch.unwrap()) {
                l.read_char();
            }
            l.input[position..l.position].to_vec()
        };

        let read_String = |l: &mut Scanner| -> Vec<char> {
            let position = l.position;
            while l.position < l.input.len()
                && l.ch.is_some()
                && !is_String_delimiter(l.ch.unwrap())
            {
                l.read_char();
            }
            l.input[position..l.position].to_vec()
        };

        let token: Token;
        if let Some(ch) = self.ch {
            token = match ch {
                t @ ':' => Token::Colon(TokenDetails {
                    row: self.row,
                    col: self.col - 1,
                    literal: vec![t],
                }),
                t @ '!' => Token::Exclamation(TokenDetails {
                    row: self.row,
                    col: self.col - 1,
                    literal: vec![t],
                }),
                t @ '+' => Token::Plus(TokenDetails {
                    row: self.row,
                    col: self.col - 1,
                    literal: vec![t],
                }),
                t @ '-' => Token::Minus(TokenDetails {
                    row: self.row,
                    col: self.col - 1,
                    literal: vec![t],
                }),
                t @ '*' => Token::Star(TokenDetails {
                    row: self.row,
                    col: self.col - 1,
                    literal: vec![t],
                }),
                t @ '%' => Token::Percentage(TokenDetails {
                    row: self.row,
                    col: self.col - 1,
                    literal: vec![t],
                }),
                t @ '\n' | t @ '\r' => {
                    let token_details = Token::EOL(TokenDetails {
                        row: self.row,
                        col: self.col - 1,
                        // one col back, the length of this literal, since read_char already advanced that length
                        literal: vec![t],
                    });
                    self.row = self.row + 1;
                    self.col = 0;
                    token_details
                }
                '"' => {
                    self.read_char();
                    let str: Vec<char> = read_String(self);
                    Token::String(TokenDetails {
                        row: self.row,
                        col: self.col,
                        literal: str,
                    })
                }
                'A'..='Z' | 'a'..='z' => {
                    let ident: Vec<char> = read_Identifier(self);
                    return Token::Identifier(TokenDetails {
                        row: self.row,
                        col: self.col - ident.len() - 1,
                        literal: ident,
                    }); // we don't want to call read_char after he
                        // match again, so we return here already
                }
                '0'..='9' => {
                    let num: Vec<char> = read_Number(self);
                    return Token::Number(TokenDetails {
                        row: self.row,
                        col: self.col - num.len() - 1,
                        literal: num,
                    }); // same here
                }
                t @ _ => Token::Illegal(TokenDetails {
                    row: self.row,
                    col: self.col - 1,
                    literal: vec![t],
                }),
            };
        } else {
            token = Token::EOF;
        }

        self.read_char();
        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assignment() {
        let input = "1:x".chars().collect();
        let expected = vec![
            Token::Number(TokenDetails {
                row: 0,
                col: 0,
                literal: vec!['1'],
            }),
            Token::Colon(TokenDetails {
                row: 0,
                col: 1,
                literal: vec![':'],
            }),
            Token::Identifier(TokenDetails {
                row: 0,
                col: 2,
                literal: vec!['x'],
            }),
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
            Token::Number(TokenDetails {
                row: 0,
                col: 0,
                literal: vec!['1', '2'],
            }),
            Token::Exclamation(TokenDetails {
                row: 0,
                col: 2,
                literal: vec!['!'],
            }),
            Token::EOF,
        ];
        let mut l = Scanner::new(input);
        let actual = l.scan();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_location_of_token() {
        let input = "1!\n1:x\n".chars().collect();
        let expected = vec![
            Token::Number(TokenDetails {
                row: 0,
                col: 0,
                literal: vec!['1'],
            }),
            Token::Exclamation(TokenDetails {
                row: 0,
                col: 1,
                literal: vec!['!'],
            }),
            Token::EOL(TokenDetails {
                row: 0,
                col: 2,
                literal: vec!['\n'],
            }),
            Token::Number(TokenDetails {
                row: 1,
                col: 0,
                literal: vec!['1'],
            }),
            Token::Colon(TokenDetails {
                row: 1,
                col: 1,
                literal: vec![':'],
            }),
            Token::Identifier(TokenDetails {
                row: 1,
                col: 2,
                literal: vec!['x'],
            }),
            Token::EOL(TokenDetails {
                row: 1,
                col: 3,
                literal: vec!['\n'],
            }),
            Token::EOF,
        ];
        let mut l = Scanner::new(input);
        let actual = l.scan();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_arithmetic_tokens() {
        let input = "12+3*2-2%2:x\n".chars().collect();
        let expected = vec![
            Token::Number(TokenDetails {
                row: 0,
                col: 0,
                literal: vec!['1', '2'],
            }),
            Token::Plus(TokenDetails {
                row: 0,
                col: 2,
                literal: vec!['+'],
            }),
            Token::Number(TokenDetails {
                row: 0,
                col: 3,
                literal: vec!['3'],
            }),
            Token::Star(TokenDetails {
                row: 0,
                col: 4,
                literal: vec!['*'],
            }),
            Token::Number(TokenDetails {
                row: 0,
                col: 5,
                literal: vec!['2'],
            }),
            Token::Minus(TokenDetails {
                row: 0,
                col: 6,
                literal: vec!['-'],
            }),
            Token::Number(TokenDetails {
                row: 0,
                col: 7,
                literal: vec!['2'],
            }),
            Token::Percentage(TokenDetails {
                row: 0,
                col: 8,
                literal: vec!['%'],
            }),
            Token::Number(TokenDetails {
                row: 0,
                col: 9,
                literal: vec!['2'],
            }),
            Token::Colon(TokenDetails {
                row: 0,
                col: 10,
                literal: vec![':'],
            }),
            Token::Identifier(TokenDetails {
                row: 0,
                col: 11,
                literal: vec!['x'],
            }),
            Token::EOL(TokenDetails {
                row: 0,
                col: 12,
                literal: vec!['\n'],
            }),
            Token::EOF,
        ];
        let mut l = Scanner::new(input);
        let actual = l.scan();
        assert_eq!(actual, expected);
    }

    fn test_multichar_tokens() {}

    fn test_whitespace_location() {}

    fn test_Illegal_tokens() {}
}
