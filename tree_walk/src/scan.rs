use std::collections::VecDeque;

#[derive(Clone, Debug, PartialEq)]
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
    Identifier(String),
    Str(String),
    Number(f32),

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

    pub fn token_type(&self) -> TokenType {
        self.type_.clone()
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.type_ {
            TokenType::LeftParen => write!(f, "("),
            TokenType::RightParen => write!(f, ")"),
            TokenType::LeftBrace => write!(f, "{{"),
            TokenType::RightBrace => write!(f, "}}"),
            TokenType::Comma => write!(f, ","),
            TokenType::Dot => write!(f, "."),
            TokenType::Minus => write!(f, "-"),
            TokenType::Plus => write!(f, "+"),
            TokenType::Semicolon => write!(f, ";"),
            TokenType::Slash => write!(f, "/"),
            TokenType::Star => write!(f, "*"),
            TokenType::Bang => write!(f, "!"),
            TokenType::BangEqual => write!(f, "!="),
            TokenType::Equal => write!(f, "="),
            TokenType::EqualEqual => write!(f, "=="),
            TokenType::Greater => write!(f, ">"),
            TokenType::GreaterEqual => write!(f, ">="),
            TokenType::Less => write!(f, "<"),
            TokenType::LessEqual => write!(f, "<="),
            TokenType::Identifier(string) => write!(f, "{}", string),
            TokenType::Str(string) => write!(f, "{}", string),
            TokenType::Number(number) => write!(f, "{}", number),
            TokenType::And => write!(f, "and"),
            TokenType::Class => write!(f, "class"),
            TokenType::Else => write!(f, "else"),
            TokenType::False => write!(f, "false"),
            TokenType::Fun => write!(f, "fun"),
            TokenType::For => write!(f, "for"),
            TokenType::If => write!(f, "if"),
            TokenType::Nil => write!(f, "nil"),
            TokenType::Or => write!(f, "or"),
            TokenType::Print => write!(f, "print"),
            TokenType::Return => write!(f, "return"),
            TokenType::Super => write!(f, "super"),
            TokenType::This => write!(f, "this"),
            TokenType::True => write!(f, "true"),
            TokenType::Var => write!(f, "var"),
            TokenType::While => write!(f, "while"),
            TokenType::EOF => write!(f, "EOF"),
        }
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
            while !self.text.is_empty() && self.text.front() != Some(&'\n') {
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
            Some(c @ '0'..='9') => {
                let mut literal = String::from(c);
                while let Some(c @ '0'..='9' | c @ '.') = self.text.front() {
                    literal.push(*c);
                    self.text.pop_front();
                }
                match literal.parse::<f32>() {
                    Ok(n) => Some(Ok(Token::new(TokenType::Number(n), self.line))),
                    _ => Some(Err(Error::NumberParse {
                        literal,
                        line: self.line,
                    })),
                }
            }
            Some(c @ 'a'..='z' | c @ 'A'..='Z' | c @ '_') => {
                let mut literal = String::from(c);
                while let Some(c @ 'a'..='z' | c @ 'A'..='Z' | c @ '_' | c @ '0'..='9') =
                    self.text.front()
                {
                    literal.push(*c);
                    self.text.pop_front();
                }
                match &literal[..] {
                    "and" => Some(Ok(Token::new(TokenType::And, self.line))),
                    "class" => Some(Ok(Token::new(TokenType::Class, self.line))),
                    "else" => Some(Ok(Token::new(TokenType::Else, self.line))),
                    "false" => Some(Ok(Token::new(TokenType::False, self.line))),
                    "for" => Some(Ok(Token::new(TokenType::For, self.line))),
                    "fun" => Some(Ok(Token::new(TokenType::Fun, self.line))),
                    "if" => Some(Ok(Token::new(TokenType::If, self.line))),
                    "nil" => Some(Ok(Token::new(TokenType::Nil, self.line))),
                    "or" => Some(Ok(Token::new(TokenType::Or, self.line))),
                    "print" => Some(Ok(Token::new(TokenType::Print, self.line))),
                    "return" => Some(Ok(Token::new(TokenType::Return, self.line))),
                    "super" => Some(Ok(Token::new(TokenType::Super, self.line))),
                    "this" => Some(Ok(Token::new(TokenType::This, self.line))),
                    "true" => Some(Ok(Token::new(TokenType::True, self.line))),
                    "var" => Some(Ok(Token::new(TokenType::Var, self.line))),
                    "while" => Some(Ok(Token::new(TokenType::While, self.line))),
                    _ => Some(Ok(Token::new(TokenType::Identifier(literal), self.line))),
                }
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
    NumberParse { literal: String, line: usize },
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

    #[test]
    fn test_number_literal() {
        assert_eq!(
            Ok(vec![
                Token::new(TokenType::Number(12.345), 0),
                Token::new(TokenType::EOF, 0)
            ]),
            scan_tokens("12.345".to_string())
        );
        assert_eq!(
            Ok(vec![
                Token::new(TokenType::Number(12345.0), 0),
                Token::new(TokenType::EOF, 0)
            ]),
            scan_tokens("12345".to_string())
        );
        assert_eq!(
            Ok(vec![
                Token::new(TokenType::Number(0.12345), 0),
                Token::new(TokenType::EOF, 0)
            ]),
            scan_tokens("0.12345".to_string())
        );
        // violates a grammar rule, but this is the correct sequence of tokens
        assert_eq!(
            Ok(vec![
                Token::new(TokenType::Dot, 0),
                Token::new(TokenType::Number(12345.0), 0),
                Token::new(TokenType::EOF, 0)
            ]),
            scan_tokens(".12345".to_string())
        );
    }

    #[test]
    fn test_identifier() {
        assert_eq!(
            Ok(vec![
                Token::new(TokenType::Identifier("orchid".to_string()), 0),
                Token::new(TokenType::EOF, 0)
            ]),
            scan_tokens("orchid".to_string())
        );
    }

    #[test]
    fn test_keywords() {
        assert_eq!(
            Ok(vec![
                Token::new(TokenType::And, 0),
                Token::new(TokenType::Class, 0),
                Token::new(TokenType::Else, 0),
                Token::new(TokenType::False, 0),
                Token::new(TokenType::For, 0),
                Token::new(TokenType::Fun, 0),
                Token::new(TokenType::If, 0),
                Token::new(TokenType::Nil, 0),
                Token::new(TokenType::Or, 0),
                Token::new(TokenType::Print, 0),
                Token::new(TokenType::Return, 0),
                Token::new(TokenType::Super, 0),
                Token::new(TokenType::This, 0),
                Token::new(TokenType::True, 0),
                Token::new(TokenType::Var, 0),
                Token::new(TokenType::While, 0),
                Token::new(TokenType::EOF, 0)
            ]),
            scan_tokens(
                "and class else false for fun if nil or print return super this true var while"
                    .to_string()
            )
        );
    }
}
