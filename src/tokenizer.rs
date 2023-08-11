#[derive(Copy, Clone)]
pub struct Tokenizer<'a> {
    input: &'a Vec<char>,
    index: usize,
}

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Identifier(Loc<'a>),
    Colon(Loc<'a>),
    Number(Loc<'a>),
    String(Loc<'a>),
    Plus(Loc<'a>),
    Minus(Loc<'a>),
    Star(Loc<'a>),
    Percentage(Loc<'a>),
    EOF,
}

#[derive(Debug, PartialEq)]
pub struct Loc<'a> {
    start: usize,
    end: usize,
    literal: &'a [char],
}

#[derive(Debug, PartialEq)]
pub enum TokenizerError {
    UnknownToken,
    UnterminatedString,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a Vec<char>) -> Self {
        Self { input, index: 0 }
    }

    pub fn errors(&self) -> Vec<TokenizerError> {
        self.collect::<Vec<_>>()
            .into_iter()
            .filter_map(|t| t.err())
            .collect()
    }

    fn read_number(&mut self) {
        while self.index < self.input.len() {
            match self.input[self.index] {
                '0'..='9' => {
                    self.index += 1;
                }
                _ => return,
            }
        }
    }

    fn read_identifier(&mut self) {
        while self.index < self.input.len() {
            match self.input[self.index] {
                'A'..='Z' | 'a'..='z' => {
                    self.index += 1;
                }
                _ => return,
            }
        }
    }

    fn read_string(&mut self) -> Result<(), TokenizerError> {
        self.index += 1;
        while self.index < self.input.len() {
            match self.input[self.index] {
                '"' => {
                    self.index += 1;
                    return Ok(());
                }
                _ => {
                    self.index += 1;
                }
            }
        }
        Err(TokenizerError::UnterminatedString)
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token<'a>, TokenizerError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.input.len() {
            self.index += 1;
            return Some(Ok(Token::EOF));
        }
        if self.index > self.input.len() {
            return None;
        }

        match self.input[self.index] {
            ':' => {
                self.index += 1;
                Some(Ok(Token::Colon(Loc {
                    start: self.index - 1,
                    end: self.index - 1,
                    literal: &self.input[self.index - 1..self.index],
                })))
            }
            '+' => {
                self.index += 1;
                Some(Ok(Token::Plus(Loc {
                    start: self.index - 1,
                    end: self.index - 1,
                    literal: &self.input[self.index - 1..self.index],
                })))
            }
            '-' => {
                self.index += 1;
                Some(Ok(Token::Minus(Loc {
                    start: self.index - 1,
                    end: self.index - 1,
                    literal: &self.input[self.index - 1..self.index],
                })))
            }
            '*' => {
                self.index += 1;
                Some(Ok(Token::Star(Loc {
                    start: self.index - 1,
                    end: self.index - 1,
                    literal: &self.input[self.index - 1..self.index],
                })))
            }
            '%' => {
                self.index += 1;
                Some(Ok(Token::Percentage(Loc {
                    start: self.index - 1,
                    end: self.index - 1,
                    literal: &self.input[self.index - 1..self.index],
                })))
            }
            '0'..='9' => {
                let start = self.index;
                self.read_number();
                let literal = &self.input[start..self.index];
                Some(Ok(Token::Number(Loc {
                    start,
                    end: self.index - 1,
                    literal,
                })))
            }
            'A'..='Z' | 'a'..='z' => {
                let start = self.index;
                self.read_identifier();
                Some(Ok(Token::Identifier(Loc {
                    start,
                    end: self.index - 1,
                    literal: &self.input[start..self.index],
                })))
            }
            '"' => {
                let start = self.index + 1;
                if let Err(e) = self.read_string() {
                    return Some(Err(e));
                };
                Some(Ok(Token::String(Loc {
                    start,
                    end: self.index - 1,
                    literal: &self.input[start..self.index - 1],
                })))
            }
            _ => {
                self.index += 1;
                Some(Err(TokenizerError::UnknownToken))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assignment() {
        let input = &"1:x".chars().collect();
        let t = Tokenizer::new(input);
        let actual: Vec<_> = t.collect();
        let expected = vec![
            Ok(Token::Number(Loc {
                start: 0,
                end: 0,
                literal: &t.input[0..1],
            })),
            Ok(Token::Colon(Loc {
                start: 1,
                end: 1,
                literal: &t.input[1..2],
            })),
            Ok(Token::Identifier(Loc {
                start: 2,
                end: 2,
                literal: &t.input[2..3],
            })),
            Ok(Token::EOF),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_multichar_identifiers() {
        let input = &"1:diameter".chars().collect();
        let t = Tokenizer::new(input);
        let actual: Vec<_> = t.collect();
        let expected = vec![
            Ok(Token::Number(Loc {
                start: 0,
                end: 0,
                literal: &t.input[0..1],
            })),
            Ok(Token::Colon(Loc {
                start: 1,
                end: 1,
                literal: &t.input[1..2],
            })),
            Ok(Token::Identifier(Loc {
                start: 2,
                end: 9,
                literal: &t.input[2..10],
            })),
            Ok(Token::EOF),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_longer_numbers() {
        let input = &"43:age".chars().collect();
        let t = Tokenizer::new(input);
        let actual: Vec<_> = t.collect();
        let expected = vec![
            Ok(Token::Number(Loc {
                start: 0,
                end: 1,
                literal: &t.input[0..2],
            })),
            Ok(Token::Colon(Loc {
                start: 2,
                end: 2,
                literal: &t.input[2..3],
            })),
            Ok(Token::Identifier(Loc {
                start: 3,
                end: 5,
                literal: &t.input[3..6],
            })),
            Ok(Token::EOF),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_strings() {
        let input = &"\"hello world\":s".chars().collect();
        let t = Tokenizer::new(input);
        let expected = vec![
            Ok(Token::String(Loc {
                start: 0,
                end: 12,
                literal: &t.input[0..13],
            })),
            Ok(Token::Colon(Loc {
                start: 13,
                end: 13,
                literal: &t.input[13..14],
            })),
            Ok(Token::Identifier(Loc {
                start: 14,
                end: 14,
                literal: &t.input[14..15],
            })),
            Ok(Token::EOF),
        ];
        let actual: Vec<_> = t.collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_arithmetic_tokens() {
        let input = &"1+2*4-2%2".chars().collect();
        let t = Tokenizer::new(input);
        let expected = vec![
            Ok(Token::Number(Loc {
                start: 0,
                end: 0,
                literal: &t.input[0..1],
            })),
            Ok(Token::Plus(Loc {
                start: 1,
                end: 1,
                literal: &t.input[1..2],
            })),
            Ok(Token::Number(Loc {
                start: 2,
                end: 2,
                literal: &t.input[2..3],
            })),
            Ok(Token::Star(Loc {
                start: 3,
                end: 3,
                literal: &t.input[3..4],
            })),
            Ok(Token::Number(Loc {
                start: 4,
                end: 4,
                literal: &t.input[4..5],
            })),
            Ok(Token::Minus(Loc {
                start: 5,
                end: 5,
                literal: &t.input[5..6],
            })),
            Ok(Token::Number(Loc {
                start: 6,
                end: 6,
                literal: &t.input[6..7],
            })),
            Ok(Token::Percentage(Loc {
                start: 7,
                end: 7,
                literal: &t.input[7..8],
            })),
            Ok(Token::Number(Loc {
                start: 8,
                end: 8,
                literal: &t.input[8..9],
            })),
            Ok(Token::EOF),
        ];
        let actual: Vec<_> = t.collect();
        assert_eq!(actual, expected);
    }
}

#[test]
fn test_errors_in_collection() {
    let input = &"⍵-12:⍺\"1".chars().collect();
    let t = Tokenizer::new(input);
    let expected = vec![
        Err(TokenizerError::UnknownToken),
        Ok(Token::Minus(Loc {
            start: 1,
            end: 1,
            literal: &t.input[1..2],
        })),
        Ok(Token::Number(Loc {
            start: 2,
            end: 3,
            literal: &t.input[2..4],
        })),
        Ok(Token::Colon(Loc {
            start: 4,
            end: 4,
            literal: &t.input[4..5],
        })),
        Err(TokenizerError::UnknownToken),
        Err(TokenizerError::UnterminatedString),
        Ok(Token::EOF),
    ];
    let actual: Vec<_> = t.collect();
    let errors = t.errors();
    assert_eq!(3, errors.len());
    assert_eq!(actual, expected);
}
