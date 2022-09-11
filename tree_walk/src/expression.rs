use crate::scan::Token;

#[derive(Debug, PartialEq)]
pub enum Expression {
    Literal {
        value: f32,
    },
    Unary {
        operator: Token,
        expression: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    Grouping {
        expression: Box<Expression>,
    },
}

pub fn print(expression: &Expression) -> String {
    match expression {
        Expression::Literal { value } => value.to_string(),
        Expression::Unary {
            operator,
            expression,
        } => parenthesize(&operator, &[expression]),
        Expression::Binary {
            left,
            operator,
            right,
        } => parenthesize(&operator, &[left, right]),
        Expression::Grouping { expression } => parenthesize(&"group".to_string(), &[expression]),
    }
}

fn parenthesize(name: &impl std::fmt::Display, expressions: &[&Expression]) -> String {
    let mut string = format!("({}", name);
    for expression in expressions {
        string = format!("{} {}", string, print(expression));
    }
    string.push(')');
    string
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scan::TokenType;

    #[test]
    fn test_expression_instances() {
        let _literal = Expression::Literal { value: 5.0 };

        let _unary = Expression::Unary {
            operator: Token::new(TokenType::Minus, 0),
            expression: Box::new(Expression::Literal { value: 5.0 }),
        };

        let _binary = Expression::Binary {
            left: Box::new(Expression::Literal { value: 5.0 }),
            operator: Token::new(TokenType::Plus, 0),
            right: Box::new(Expression::Literal { value: 6.0 }),
        };

        let _grouping = Expression::Grouping {
            expression: Box::new(Expression::Literal { value: 5.0 }),
        };
    }

    #[test]
    fn test_print_literal() {
        let expected = "5".to_string();
        let literal = Expression::Literal { value: 5.0 };

        assert_eq!(expected, print(&literal));
    }

    #[test]
    fn test_print_unary() {
        let unary = Expression::Unary {
            operator: Token::new(TokenType::Minus, 0),
            expression: Box::new(Expression::Literal { value: 5.0 }),
        };
        assert_eq!("(- 5)".to_string(), print(&unary));
    }

    #[test]
    fn test_print_binary() {
        let binary = Expression::Binary {
            left: Box::new(Expression::Literal { value: 5.0 }),
            operator: Token::new(TokenType::Minus, 0),
            right: Box::new(Expression::Literal { value: 6.0 }),
        };
        assert_eq!("(- 5 6)", print(&binary));
    }

    #[test]
    fn test_print_grouping() {
        let grouping = Expression::Grouping {
            expression: Box::new(Expression::Literal { value: 5.0 }),
        };
        assert_eq!("(group 5)".to_string(), print(&grouping));
    }

    #[test]
    fn test_example() {
        let expression = Expression::Binary {
            left: Box::new(Expression::Unary {
                operator: Token::new(TokenType::Minus, 0),
                expression: Box::new(Expression::Literal { value: 123.00 }),
            }),
            operator: Token::new(TokenType::Star, 0),
            right: Box::new(Expression::Grouping {
                expression: Box::new(Expression::Literal { value: 45.67 }),
            }),
        };
        assert_eq!("(* (- 123) (group 45.67))".to_string(), print(&expression));
    }
}
