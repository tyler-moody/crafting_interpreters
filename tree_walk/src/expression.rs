use crate::scan::Token;

pub enum Expression<'a> {
    Literal {
        value: Box<dyn std::fmt::Display>,
    },
    Unary {
        operator: Token,
        expression: &'a Expression<'a>,
    },
    Binary {
        left: &'a Expression<'a>,
        operator: Token,
        right: &'a Expression<'a>,
    },
    Grouping {
        expression: &'a Expression<'a>,
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
        let foo = Expression::Literal {
            value: Box::new("foo".to_string()),
        };

        let _unary = Expression::Unary {
            operator: Token::new(TokenType::Minus, 0),
            expression: &foo,
        };

        let bar = Expression::Literal {
            value: Box::new("bar".to_string()),
        };

        let binary = Expression::Binary {
            left: &foo,
            operator: Token::new(TokenType::Plus, 0),
            right: &bar,
        };

        let _grouping = Expression::Grouping {
            expression: &binary,
        };
    }

    #[test]
    fn test_print_literal() {
        let expected = "foo".to_string();
        let literal = Expression::Literal {
            value: Box::new(expected.clone()),
        };

        assert_eq!(expected, print(&literal));
    }

    #[test]
    fn test_print_unary() {
        let unary = Expression::Unary {
            operator: Token::new(TokenType::Minus, 0),
            expression: &Expression::Literal {
                value: Box::new("foo".to_string()),
            },
        };
        assert_eq!("(- foo)".to_string(), print(&unary));
    }

    #[test]
    fn test_print_binary() {
        let binary = Expression::Binary {
            left: &Expression::Literal {
                value: Box::new("foo".to_string()),
            },
            operator: Token::new(TokenType::Minus, 0),
            right: &Expression::Literal {
                value: Box::new("bar".to_string()),
            },
        };
        assert_eq!("(- foo bar)", print(&binary));
    }

    #[test]
    fn test_print_grouping() {
        let grouping = Expression::Grouping {
            expression: &Expression::Literal {
                value: Box::new("foo".to_string()),
            },
        };
        assert_eq!("(group foo)".to_string(), print(&grouping));
    }

    #[test]
    fn test_example() {
        let expression = Expression::Binary {
            left: &Expression::Unary {
                operator: Token::new(TokenType::Minus, 0),
                expression: &Expression::Literal {
                    value: Box::new(123),
                },
            },
            operator: Token::new(TokenType::Star, 0),
            right: &Expression::Grouping {
                expression: &Expression::Literal {
                    value: Box::new(45.67),
                },
            },
        };
        assert_eq!("(* (- 123) (group 45.67))".to_string(), print(&expression));
    }
}
