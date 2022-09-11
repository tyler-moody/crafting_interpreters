use crate::expression::Expression;
use crate::expression::Value::*;
use crate::scan::{Token, TokenType};
use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub enum Error {
    Placeholder,
    UnclosedParen,
}

pub fn parse(mut tokens: VecDeque<Token>) -> Result<Vec<Box<Expression>>, Error> {
    let mut expressions = Vec::new();
    while tokens.len() > 0 {
        expressions.push(expression(&mut tokens)?);
    }
    Ok(expressions)
}

fn expression(tokens: &mut VecDeque<Token>) -> Result<Box<Expression>, Error> {
    equality(tokens)
}

fn equality(tokens: &mut VecDeque<Token>) -> Result<Box<Expression>, Error> {
    let mut expr = comparison(tokens)?;

    let operators = [TokenType::EqualEqual, TokenType::BangEqual];
    while tokens.len() > 0 && operators.contains(&tokens[0].token_type()) {
        let operator = tokens.pop_front().ok_or(Error::Placeholder)?;
        let right = comparison(tokens)?;
        expr = Box::new(Expression::Binary {
            left: expr,
            operator,
            right,
        });
    }
    Ok(expr)
}

fn comparison(tokens: &mut VecDeque<Token>) -> Result<Box<Expression>, Error> {
    let mut expr = term(tokens)?;

    let operators = [
        TokenType::Greater,
        TokenType::GreaterEqual,
        TokenType::Less,
        TokenType::LessEqual,
    ];
    while tokens.len() > 0 && operators.contains(&tokens[0].token_type()) {
        let operator = tokens.pop_front().ok_or(Error::Placeholder)?;
        let right = term(tokens)?;
        expr = Box::new(Expression::Binary {
            left: expr,
            operator,
            right,
        })
    }
    Ok(expr)
}

fn term(tokens: &mut VecDeque<Token>) -> Result<Box<Expression>, Error> {
    let mut expr = factor(tokens)?;

    let operators = [TokenType::Minus, TokenType::Plus];
    while tokens.len() > 0 && operators.contains(&tokens[0].token_type()) {
        let operator = tokens.pop_front().ok_or(Error::Placeholder)?;
        let right = factor(tokens)?;
        expr = Box::new(Expression::Binary {
            left: expr,
            operator,
            right,
        })
    }
    Ok(expr)
}

fn factor(tokens: &mut VecDeque<Token>) -> Result<Box<Expression>, Error> {
    let mut expr = unary(tokens)?;

    let operators = [TokenType::Slash, TokenType::Star];
    while tokens.len() > 0 && operators.contains(&tokens[0].token_type()) {
        let operator = tokens.pop_front().ok_or(Error::Placeholder)?;
        let right = unary(tokens)?;
        expr = Box::new(Expression::Binary {
            left: expr,
            operator,
            right,
        })
    }
    Ok(expr)
}

fn unary(tokens: &mut VecDeque<Token>) -> Result<Box<Expression>, Error> {
    let operators = [TokenType::Minus, TokenType::Bang];

    if tokens.len() > 0 && operators.contains(&tokens[0].token_type()) {
        let operator = tokens.pop_front().ok_or(Error::Placeholder)?;
        let inner = primary(tokens)?;
        return Ok(Box::new(Expression::Unary {
            operator,
            expression: inner,
        }));
    }
    primary(tokens)
}

fn primary(tokens: &mut VecDeque<Token>) -> Result<Box<Expression>, Error> {
    if tokens.len() < 1 {
        return Err(Error::Placeholder);
    }
    match tokens.pop_front().ok_or(Error::Placeholder)?.token_type() {
        TokenType::LeftParen => {
            let expression = expression(tokens)?;
            let closing_paren = tokens.pop_front().ok_or(Error::UnclosedParen)?;
            if closing_paren.token_type() != TokenType::RightParen {
                return Err(Error::UnclosedParen);
            }
            Ok(Box::new(Expression::Grouping { expression }))
        }
        TokenType::Number(n) => Ok(Box::new(Expression::Literal { value: Float(n) })),
        TokenType::Str(s) => Ok(Box::new(Expression::Literal { value: Str(s) })),
        TokenType::True => Ok(Box::new(Expression::Literal { value: True })),
        TokenType::False => Ok(Box::new(Expression::Literal { value: False })),
        _ => Err(Error::Placeholder),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scan::{Token, TokenType};

    #[test]
    fn test_placeholder() {
        let mut tokens = VecDeque::new();
        tokens.push_back(Token::new(TokenType::EOF, 0));
        assert_eq!(Err(Error::Placeholder), parse(tokens));
    }

    #[test]
    fn test_literal() {
        let mut tokens = VecDeque::new();
        tokens.push_back(Token::new(TokenType::Number(5.0), 0));
        let expected = Box::new(Expression::Literal { value: Float(5.0) });
        assert_eq!(Ok(vec![expected]), parse(tokens));

        tokens = VecDeque::new();
        tokens.push_back(Token::new(TokenType::Str("foo".to_string()), 0));
        let expected = Box::new(Expression::Literal {
            value: Str("foo".to_string()),
        });
        assert_eq!(Ok(vec![expected]), parse(tokens));

        tokens = VecDeque::new();
        tokens.push_back(Token::new(TokenType::False, 0));
        let expected = Box::new(Expression::Literal { value: False });
        assert_eq!(Ok(vec![expected]), parse(tokens));

        tokens = VecDeque::new();
        tokens.push_back(Token::new(TokenType::True, 0));
        let expected = Box::new(Expression::Literal { value: True });
        assert_eq!(Ok(vec![expected]), parse(tokens));
    }

    #[test]
    fn test_unary() {
        let mut tokens = VecDeque::new();
        tokens.push_back(Token::new(TokenType::Minus, 0));
        tokens.push_back(Token::new(TokenType::Number(5.0), 0));
        let expected = Box::new(Expression::Unary {
            operator: Token::new(TokenType::Minus, 0),
            expression: Box::new(Expression::Literal { value: Float(5.0) }),
        });
        assert_eq!(Ok(vec![expected]), parse(tokens));

        tokens = VecDeque::new();
        tokens.push_back(Token::new(TokenType::Bang, 0));
        tokens.push_back(Token::new(TokenType::True, 0));
        let expected = Box::new(Expression::Unary {
            operator: Token::new(TokenType::Bang, 0),
            expression: Box::new(Expression::Literal { value: True }),
        });
        assert_eq!(Ok(vec![expected]), parse(tokens));
    }

    fn test_binary(type_: TokenType) {
        let mut tokens = VecDeque::new();
        tokens.push_back(Token::new(TokenType::Number(5.0), 0));
        tokens.push_back(Token::new(type_.clone(), 0));
        tokens.push_back(Token::new(TokenType::Number(6.0), 0));
        let expected = Box::new(Expression::Binary {
            left: Box::new(Expression::Literal { value: Float(5.0) }),
            operator: Token::new(type_, 0),
            right: Box::new(Expression::Literal { value: Float(6.0) }),
        });
        assert_eq!(Ok(vec![expected]), parse(tokens));
    }

    #[test]
    fn test_factor() {
        test_binary(TokenType::Star);
        test_binary(TokenType::Slash);
    }

    #[test]
    fn test_term() {
        test_binary(TokenType::Plus);
        test_binary(TokenType::Minus);
    }

    #[test]
    fn test_comparison() {
        test_binary(TokenType::Greater);
        test_binary(TokenType::GreaterEqual);
        test_binary(TokenType::Less);
        test_binary(TokenType::LessEqual);
    }

    #[test]
    fn test_equality() {
        test_binary(TokenType::EqualEqual);
        test_binary(TokenType::BangEqual);
    }

    #[test]
    fn test_grouping() {
        let mut tokens = VecDeque::new();
        tokens.push_back(Token::new(TokenType::LeftParen, 0));
        tokens.push_back(Token::new(TokenType::Number(5.0), 0));
        tokens.push_back(Token::new(TokenType::RightParen, 0));
        let expected = Box::new(Expression::Grouping {
            expression: Box::new(Expression::Literal { value: Float(5.0) }),
        });
        assert_eq!(Ok(vec![expected]), parse(tokens));
    }

    #[test]
    fn test_grouped_expression() {
        let mut tokens = VecDeque::new();
        tokens.push_back(Token::new(TokenType::LeftParen, 0));
        tokens.push_back(Token::new(TokenType::Number(5.0), 0));
        tokens.push_back(Token::new(TokenType::Star, 0));
        tokens.push_back(Token::new(TokenType::Number(6.0), 0));
        tokens.push_back(Token::new(TokenType::RightParen, 0));
        let expected = Box::new(Expression::Grouping {
            expression: Box::new(Expression::Binary {
                left: Box::new(Expression::Literal { value: Float(5.0) }),
                operator: Token::new(TokenType::Star, 0),
                right: Box::new(Expression::Literal { value: Float(6.0) }),
            }),
        });
        assert_eq!(Ok(vec![expected]), parse(tokens));
    }

    #[test]
    fn test_grouping_unclosed() {
        let mut tokens = VecDeque::new();
        tokens.push_back(Token::new(TokenType::LeftParen, 0));
        tokens.push_back(Token::new(TokenType::Number(5.0), 0));
        assert_eq!(Err(Error::UnclosedParen), parse(tokens));
    }

    #[test]
    fn test_grouping_unclosed_more_tokens() {
        let mut tokens = VecDeque::new();
        tokens.push_back(Token::new(TokenType::LeftParen, 0));
        tokens.push_back(Token::new(TokenType::Number(5.0), 0));
        tokens.push_back(Token::new(TokenType::Number(5.0), 0));
        assert_eq!(Err(Error::UnclosedParen), parse(tokens));
    }
}
