use crate::scan::Token;

#[derive(Debug, PartialEq)]
pub enum Value {
    Str(String),
    Int(i64),
    Float(f32),
    False,
    True,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Str(s) => write!(f, "{}", s),
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(g) => write!(f, "{}", g),
            Value::False => write!(f, "false"),
            Value::True => write!(f, "true"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Literal {
        value: Value,
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
        let _literal = Expression::Literal {
            value: Value::Float(5.0),
        };

        let _unary = Expression::Unary {
            operator: Token::new(TokenType::Minus, 0),
            expression: Box::new(Expression::Literal {
                value: Value::Int(5),
            }),
        };

        let _binary = Expression::Binary {
            left: Box::new(Expression::Literal {
                value: Value::Int(5),
            }),
            operator: Token::new(TokenType::Plus, 0),
            right: Box::new(Expression::Literal {
                value: Value::Int(6),
            }),
        };

        let _grouping = Expression::Grouping {
            expression: Box::new(Expression::Literal {
                value: Value::Int(5),
            }),
        };
    }

    #[test]
    fn test_print_literal() {
        let expected = "5".to_string();
        let literal = Expression::Literal {
            value: Value::Int(5),
        };

        assert_eq!(expected, print(&literal));
    }

    #[test]
    fn test_print_unary() {
        let unary = Expression::Unary {
            operator: Token::new(TokenType::Minus, 0),
            expression: Box::new(Expression::Literal {
                value: Value::Int(5),
            }),
        };
        assert_eq!("(- 5)".to_string(), print(&unary));
    }

    #[test]
    fn test_print_binary() {
        let binary = Expression::Binary {
            left: Box::new(Expression::Literal {
                value: Value::Int(5),
            }),
            operator: Token::new(TokenType::Minus, 0),
            right: Box::new(Expression::Literal {
                value: Value::Int(6),
            }),
        };
        assert_eq!("(- 5 6)", print(&binary));
    }

    #[test]
    fn test_print_grouping() {
        let grouping = Expression::Grouping {
            expression: Box::new(Expression::Literal {
                value: Value::Int(5),
            }),
        };
        assert_eq!("(group 5)".to_string(), print(&grouping));
    }

    #[test]
    fn test_example() {
        let expression = Expression::Binary {
            left: Box::new(Expression::Unary {
                operator: Token::new(TokenType::Minus, 0),
                expression: Box::new(Expression::Literal {
                    value: Value::Int(123),
                }),
            }),
            operator: Token::new(TokenType::Star, 0),
            right: Box::new(Expression::Grouping {
                expression: Box::new(Expression::Literal {
                    value: Value::Float(45.67),
                }),
            }),
        };
        assert_eq!("(* (- 123) (group 45.67))".to_string(), print(&expression));
    }
}
