use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    // single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // one-or-two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // literals
    Identifier,
    Str(String),
    Number,

    // keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    type_: TokenType,
    line: usize,
}

impl Token {
    pub fn new(type_: TokenType, line: usize) -> Self {
        Self { type_, line }
    }
}

struct Source {
    text: VecDeque<char>,
    line: usize,
    eof_sent: bool,
}

impl Source {
    pub fn new(source: String) -> Self {
        let mut text = VecDeque::new();
        for c in source.chars() {
            text.push_back(c);
        }
        Self {
            text,
            line: 0,
            eof_sent: false,
        }
    }
}

impl Iterator for Source {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.text.len() > 1 && self.text[0] == '/' && self.text[1] == '/' {
            while self.text.len() > 0 && self.text.front() != Some(&'\n') {
                self.text.pop_front();
            }
        }

        loop {
            match self.text.front() {
                Some('\n') => {
                    self.line += 1;
                    self.text.pop_front();
                }
                Some(' ') | Some('\r') | Some('\t') => {
                    self.text.pop_front();
                }
                _ => {
                    break;
                }
            }
        }

        match self.text.pop_front() {
            Some('(') => Some(Ok(Token::new(TokenType::LeftParen, self.line))),
            Some(')') => Some(Ok(Token::new(TokenType::RightParen, self.line))),
            Some('{') => Some(Ok(Token::new(TokenType::LeftBrace, self.line))),
            Some('}') => Some(Ok(Token::new(TokenType::RightBrace, self.line))),
            Some(',') => Some(Ok(Token::new(TokenType::Comma, self.line))),
            Some('.') => Some(Ok(Token::new(TokenType::Dot, self.line))),
            Some('-') => Some(Ok(Token::new(TokenType::Minus, self.line))),
            Some('+') => Some(Ok(Token::new(TokenType::Plus, self.line))),
            Some(';') => Some(Ok(Token::new(TokenType::Semicolon, self.line))),
            Some('*') => Some(Ok(Token::new(TokenType::Star, self.line))),
            Some('!') => match self.text.front() {
                Some('=') => {
                    self.text.pop_front();
                    Some(Ok(Token::new(TokenType::BangEqual, self.line)))
                }
                _ => Some(Ok(Token::new(TokenType::Bang, self.line))),
            },
            Some('=') => match self.text.front() {
                Some('=') => {
                    self.text.pop_front();
                    Some(Ok(Token::new(TokenType::EqualEqual, self.line)))
                }
                _ => Some(Ok(Token::new(TokenType::Equal, self.line))),
            },
            Some('>') => match self.text.front() {
                Some('=') => {
                    self.text.pop_front();
                    Some(Ok(Token::new(TokenType::GreaterEqual, self.line)))
                }
                _ => Some(Ok(Token::new(TokenType::Greater, self.line))),
            },
            Some('<') => match self.text.front() {
                Some('=') => {
                    self.text.pop_front();
                    Some(Ok(Token::new(TokenType::LessEqual, self.line)))
                }
                _ => Some(Ok(Token::new(TokenType::Less, self.line))),
            },
            Some('/') => Some(Ok(Token::new(TokenType::Slash, self.line))),
            Some('"') => {
                let mut literal = String::new();
                loop {
                    match self.text.pop_front() {
                        Some('\n') => {
                            self.line += 1;
                        }
                        Some('"') => {
                            break;
                        }
                        Some(c) => {
                            literal.push(c);
                        }
                        None => {
                            return Some(Err(Error::UnterminatedString { line: self.line }));
                        }
                    }
                }
                Some(Ok(Token::new(TokenType::Str(literal), self.line)))
            }
            Some(c) => Some(Err(Error::BadChar { c, line: self.line })),
            None => {
                if !self.eof_sent {
                    self.eof_sent = true;
                    Some(Ok(Token::new(TokenType::EOF, self.line)))
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    BadChar { c: char, line: usize },
    UnterminatedString { line: usize },
}

pub fn scan_tokens(source: String) -> Result<Vec<Token>, Error> {
    let tokenizer = Source::new(source);
    tokenizer.collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_source_eof() {
        assert_eq!(
            Ok(vec![Token::new(TokenType::EOF, 0)]),
            scan_tokens("".to_string())
        );
    }

    #[test]
    fn test_single_character_operators() {
        let source = "(){},.-+;*".to_string();
        let actual = scan_tokens(source);
        let expected = Ok(vec![
            Token::new(TokenType::LeftParen, 0),
            Token::new(TokenType::RightParen, 0),
            Token::new(TokenType::LeftBrace, 0),
            Token::new(TokenType::RightBrace, 0),
            Token::new(TokenType::Comma, 0),
            Token::new(TokenType::Dot, 0),
            Token::new(TokenType::Minus, 0),
            Token::new(TokenType::Plus, 0),
            Token::new(TokenType::Semicolon, 0),
            Token::new(TokenType::Star, 0),
            Token::new(TokenType::EOF, 0),
        ]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_one_or_two_character_operators() {
        assert_eq!(
            Ok(vec![
                Token::new(TokenType::Bang, 0),
                Token::new(TokenType::EOF, 0)
            ]),
            scan_tokens("!".to_string())
        );
        assert_eq!(
            Ok(vec![
                Token::new(TokenType::BangEqual, 0),
                Token::new(TokenType::EOF, 0)
            ]),
            scan_tokens("!=".to_string())
        );

        assert_eq!(
            Ok(vec![
                Token::new(TokenType::Equal, 0),
                Token::new(TokenType::EOF, 0)
            ]),
            scan_tokens("=".to_string())
        );
        assert_eq!(
            Ok(vec![
                Token::new(TokenType::EqualEqual, 0),
                Token::new(TokenType::EOF, 0)
            ]),
            scan_tokens("==".to_string())
        );

        assert_eq!(
            Ok(vec![
                Token::new(TokenType::Greater, 0),
                Token::new(TokenType::EOF, 0)
            ]),
            scan_tokens(">".to_string())
        );
        assert_eq!(
            Ok(vec![
                Token::new(TokenType::GreaterEqual, 0),
                Token::new(TokenType::EOF, 0)
            ]),
            scan_tokens(">=".to_string())
        );

        assert_eq!(
            Ok(vec![
                Token::new(TokenType::Less, 0),
                Token::new(TokenType::EOF, 0)
            ]),
            scan_tokens("<".to_string())
        );
        assert_eq!(
            Ok(vec![
                Token::new(TokenType::LessEqual, 0),
                Token::new(TokenType::EOF, 0)
            ]),
            scan_tokens("<=".to_string())
        );

        assert_eq!(
            Ok(vec![
                Token::new(TokenType::Slash, 0),
                Token::new(TokenType::EOF, 0)
            ]),
            scan_tokens("/".to_string())
        );
        assert_eq!(
            Ok(vec![Token::new(TokenType::EOF, 0)]),
            scan_tokens("//".to_string())
        );
    }

    #[test]
    fn test_whitespace() {
        assert_eq!(
            Ok(vec![
                Token::new(TokenType::Slash, 1),
                Token::new(TokenType::EOF, 1)
            ]),
            scan_tokens("//\n \t/".to_string())
        );
    }

    #[test]
    fn test_bad_character() {
        assert_eq!(
            Err(Error::BadChar { c: '&', line: 0 }),
            scan_tokens("&".to_string())
        );
    }

    #[test]
    fn test_string_literal() {
        assert_eq!(
            Ok(vec![
                Token::new(TokenType::Str("foo".to_string()), 0),
                Token::new(TokenType::EOF, 0)
            ]),
            scan_tokens("\"foo\"".to_string())
        );
    }

    #[test]
    fn test_unterminated_string_literal() {
        assert_eq!(
            Err(Error::UnterminatedString { line: 0 }),
            scan_tokens("\"foo".to_string())
        );
    }

    #[test]
    fn test_unterminated_multiline_string_literal() {
        assert_eq!(
            Err(Error::UnterminatedString { line: 3 }),
            scan_tokens("\"foo\n\n\nbar".to_string())
        );
    }
}
