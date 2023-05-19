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
}

#[derive(Debug, PartialEq)]
pub struct Loc<'a> {
    start: usize,
    end: usize,
    literal: &'a [char],
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a Vec<char>) -> Self {
        Self {
            input: &input,
            index: 0,
        }
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

    // TODO: This should actually return an error since we might be dealing with an unbalanced
    // string (no closing character)
    fn read_string(&mut self) {
        self.index += 1;
        while self.index < self.input.len() {
            match self.input[self.index] {
                '"' => {
                    self.index += 1;
                    return;
                }
                _ => {
                    self.index += 1;
                }
            }
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.input.len() {
            return None;
        }

        match self.input[self.index] {
            ':' => {
                self.index += 1;
                Some(Token::Colon(Loc {
                    start: self.index - 1,
                    end: self.index - 1,
                    literal: &self.input[self.index - 1..self.index],
                }))
            }
            '+' => {
                self.index += 1;
                Some(Token::Plus(Loc {
                    start: self.index - 1,
                    end: self.index - 1,
                    literal: &self.input[self.index - 1..self.index],
                }))
            }
            '-' => {
                self.index += 1;
                Some(Token::Minus(Loc {
                    start: self.index - 1,
                    end: self.index - 1,
                    literal: &self.input[self.index - 1..self.index],
                }))
            }
            '*' => {
                self.index += 1;
                Some(Token::Star(Loc {
                    start: self.index - 1,
                    end: self.index - 1,
                    literal: &self.input[self.index - 1..self.index],
                }))
            }
            '%' => {
                self.index += 1;
                Some(Token::Percentage(Loc {
                    start: self.index - 1,
                    end: self.index - 1,
                    literal: &self.input[self.index - 1..self.index],
                }))
            }
            '0'..='9' => {
                let start = self.index;
                self.read_number();
                let literal = &self.input[start..self.index];
                Some(Token::Number(Loc {
                    start,
                    end: self.index - 1,
                    literal,
                }))
            }
            'A'..='Z' | 'a'..='z' => {
                let start = self.index;
                self.read_identifier();
                Some(Token::Identifier(Loc {
                    start,
                    end: self.index - 1,
                    literal: &self.input[start..self.index],
                }))
            }
            '"' => {
                let start = self.index;
                self.read_string();
                Some(Token::String(Loc {
                    start,
                    end: self.index - 1,
                    literal: &self.input[start..self.index],
                }))
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests2 {
    use super::*;

    #[test]
    fn test_assignment() {
        let input = &"1:x".chars().collect();
        let t = Tokenizer::new(input);
        let actual: Vec<Token> = t.collect();
        let expected = vec![
            Token::Number(Loc {
                start: 0,
                end: 0,
                literal: &t.input[0..1],
            }),
            Token::Colon(Loc {
                start: 1,
                end: 1,
                literal: &t.input[1..2],
            }),
            Token::Identifier(Loc {
                start: 2,
                end: 2,
                literal: &t.input[2..3],
            }),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_multichar_identifiers() {
        let input = &"1:diameter".chars().collect();
        let t = Tokenizer::new(input);
        let actual: Vec<Token> = t.collect();
        let expected = vec![
            Token::Number(Loc {
                start: 0,
                end: 0,
                literal: &t.input[0..1],
            }),
            Token::Colon(Loc {
                start: 1,
                end: 1,
                literal: &t.input[1..2],
            }),
            Token::Identifier(Loc {
                start: 2,
                end: 9,
                literal: &t.input[2..10],
            }),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_longer_numbers() {
        let input = &"43:age".chars().collect();
        let t = Tokenizer::new(input);
        let actual: Vec<Token> = t.collect();
        let expected = vec![
            Token::Number(Loc {
                start: 0,
                end: 1,
                literal: &t.input[0..2],
            }),
            Token::Colon(Loc {
                start: 2,
                end: 2,
                literal: &t.input[2..3],
            }),
            Token::Identifier(Loc {
                start: 3,
                end: 5,
                literal: &t.input[3..6],
            }),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_strings() {
        let input = &"\"hello world\":s".chars().collect();
        let t = Tokenizer::new(input);
        let expected = vec![
            Token::String(Loc {
                start: 0,
                end: 12,
                literal: &t.input[0..13],
            }),
            Token::Colon(Loc {
                start: 13,
                end: 13,
                literal: &t.input[13..14],
            }),
            Token::Identifier(Loc {
                start: 14,
                end: 14,
                literal: &t.input[14..15],
            }),
        ];
        let actual: Vec<Token> = t.collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_arithmetic_tokens() {
        let input = &"1+2*4-2%2".chars().collect();
        let t = Tokenizer::new(input);
        let expected = vec![
            Token::Number(Loc {
                start: 0,
                end: 0,
                literal: &t.input[0..1],
            }),
            Token::Plus(Loc {
                start: 1,
                end: 1,
                literal: &t.input[1..2],
            }),
            Token::Number(Loc {
                start: 2,
                end: 2,
                literal: &t.input[2..3],
            }),
            Token::Star(Loc {
                start: 3,
                end: 3,
                literal: &t.input[3..4],
            }),
            Token::Number(Loc {
                start: 4,
                end: 4,
                literal: &t.input[4..5],
            }),
            Token::Minus(Loc {
                start: 5,
                end: 5,
                literal: &t.input[5..6],
            }),
            Token::Number(Loc {
                start: 6,
                end: 6,
                literal: &t.input[6..7],
            }),
            Token::Percentage(Loc {
                start: 7,
                end: 7,
                literal: &t.input[7..8],
            }),
            Token::Number(Loc {
                start: 8,
                end: 8,
                literal: &t.input[8..9],
            }),
        ];
        let actual: Vec<Token> = t.collect();
        assert_eq!(actual, expected);
    }
}
