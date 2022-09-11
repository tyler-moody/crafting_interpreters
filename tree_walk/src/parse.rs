use crate::expression::Expression;
use crate::scan::{Token, TokenType};
use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub enum Error {
    Placeholder,
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
        TokenType::Number(n) => Ok(Box::new(Expression::Literal { value: n })),
        // TODO grouping
        // TODO what to do about true, false, nil, string values?
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
}
