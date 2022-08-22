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
    BangBang,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // literals
    Identifier,
    Str,
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

#[derive(Debug, PartialEq)]
pub enum Error {
    OutOfBounds,
    BadChar { c: char, line: usize },
}

pub fn scan_tokens(source: String) -> Result<Vec<Token>, Error> {
    let mut tokens = Vec::new();
    let mut current = 0;
    let line = 0;
    while current < source.chars().count() {
        // start = current;

        let c = source.chars().nth(current).ok_or(Error::OutOfBounds)?;
        current += 1;

        match c {
            '(' => tokens.push(Token::new(TokenType::LeftParen, line)),
            ')' => tokens.push(Token::new(TokenType::RightParen, line)),
            '{' => tokens.push(Token::new(TokenType::LeftBrace, line)),
            '}' => tokens.push(Token::new(TokenType::RightBrace, line)),
            ',' => tokens.push(Token::new(TokenType::Comma, line)),
            '.' => tokens.push(Token::new(TokenType::Dot, line)),
            '-' => tokens.push(Token::new(TokenType::Minus, line)),
            '+' => tokens.push(Token::new(TokenType::Plus, line)),
            ';' => tokens.push(Token::new(TokenType::Semicolon, line)),
            '*' => tokens.push(Token::new(TokenType::Star, line)),

            c => return Err(Error::BadChar { c, line }),
        }
    }
    tokens.push(Token::new(TokenType::EOF, line));
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_character_literals() {
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
    fn test_bad_character() {
        assert_eq!(
            Err(Error::BadChar { c: '&', line: 0 }),
            scan_tokens("&".to_string())
        );
    }
}
