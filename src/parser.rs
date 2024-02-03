use crate::lexer::{Lex, Lexer, Token, TokenType};

#[derive(Debug, PartialEq)]
pub struct InnerTokenExpression {
    token: Token,
}

/// Expression Expression Operation
#[derive(Debug, PartialEq)]
pub struct InnerExpExpOp {
    expression_1: Box<Expression>,
    expression_2: Box<Expression>,
    operation: Box<Operation>,
}

#[derive(Debug, PartialEq)]
pub struct InnerAssignment {
    expression_1: Box<Expression>,
    expression_2: Box<Expression>,
    expression_3: Box<Expression>,
    operation: Box<Operation>,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    TokenExpression(InnerTokenExpression),
    ExpExpOp(InnerExpExpOp),
    Assignment(InnerAssignment),
    None,
}

#[derive(Debug, PartialEq)]
pub struct InnerTokenOperation {
    token: Token,
}

#[derive(Debug, PartialEq)]
pub enum Operation {
    TokenOperation(InnerTokenOperation),
}

fn create_assignment(
    arg1: Box<Expression>,
    arg2: Box<Expression>,
    arg3: Box<Expression>,
    operation: Box<Operation>,
) -> Expression {
    println!("{:?} {:?} {:?} {:?}", arg1, arg2, arg3, operation);

    return *arg3;
}

fn run_operation(
    arg1: Box<Expression>,
    arg2: Box<Expression>,
    operation: Box<Operation>,
) -> Expression {
    match (*arg1, *arg2) {
        // ---
        // TWO TOKENS
        (Expression::TokenExpression(a), Expression::TokenExpression(b)) => {
            match (a.token.token_type, b.token.token_type) {
                // ---
                // TWO NUMBER LITERALS
                (TokenType::NumericIntLiteral, TokenType::NumericIntLiteral) => {
                    let a_int = a.token.value.parse::<i32>().unwrap();
                    let b_int = b.token.value.parse::<i32>().unwrap();

                    let op = *operation;

                    match op {
                        Operation::TokenOperation(c) => match c.token.token_type {
                            TokenType::Addition => {
                                return Expression::TokenExpression(InnerTokenExpression {
                                    token: Token {
                                        token_type: TokenType::NumericIntLiteral,
                                        value: format!("{}", a_int + b_int),
                                    },
                                })
                            }

                            TokenType::Subtraction => {
                                return Expression::TokenExpression(InnerTokenExpression {
                                    token: Token {
                                        token_type: TokenType::NumericIntLiteral,
                                        value: format!("{}", a_int - b_int),
                                    },
                                })
                            }

                            TokenType::Multiplication => {
                                return Expression::TokenExpression(InnerTokenExpression {
                                    token: Token {
                                        token_type: TokenType::NumericIntLiteral,
                                        value: format!("{}", a_int * b_int),
                                    },
                                })
                            }

                            TokenType::Division => {
                                return Expression::TokenExpression(InnerTokenExpression {
                                    token: Token {
                                        token_type: TokenType::NumericIntLiteral,
                                        value: format!("{}", a_int / b_int),
                                    },
                                })
                            }
                            _ => Expression::None,
                        },
                    }
                }

                // ---
                // TWO NUMBER LITERALS
                (TokenType::NumericDecLiteral, TokenType::NumericDecLiteral) => {
                    let a_float = a.token.value.parse::<f32>().unwrap();
                    let b_float = b.token.value.parse::<f32>().unwrap();

                    let op = *operation;

                    match op {
                        Operation::TokenOperation(c) => match c.token.token_type {
                            TokenType::Addition => {
                                return Expression::TokenExpression(InnerTokenExpression {
                                    token: Token {
                                        token_type: TokenType::NumericIntLiteral,
                                        value: format!("{}", a_float + b_float),
                                    },
                                })
                            }

                            TokenType::Subtraction => {
                                return Expression::TokenExpression(InnerTokenExpression {
                                    token: Token {
                                        token_type: TokenType::NumericIntLiteral,
                                        value: format!("{}", a_float - b_float),
                                    },
                                })
                            }

                            TokenType::Multiplication => {
                                return Expression::TokenExpression(InnerTokenExpression {
                                    token: Token {
                                        token_type: TokenType::NumericIntLiteral,
                                        value: format!("{}", a_float * b_float),
                                    },
                                })
                            }

                            TokenType::Division => {
                                return Expression::TokenExpression(InnerTokenExpression {
                                    token: Token {
                                        token_type: TokenType::NumericIntLiteral,
                                        value: format!("{}", a_float / b_float),
                                    },
                                })
                            }

                            _ => Expression::None,
                        },
                    }
                }

                _ => Expression::None,
            }
        }

        _ => Expression::None,
    }
}

pub fn tokens_to_expr(tokens: Vec<Token>) -> Expression {
    if tokens.len() == 3 {
        let b = Expression::ExpExpOp({
            InnerExpExpOp {
                expression_1: Box::new(Expression::TokenExpression(InnerTokenExpression {
                    token: tokens[0].clone(),
                })),

                expression_2: Box::new(Expression::TokenExpression(InnerTokenExpression {
                    token: tokens[1].clone(),
                })),

                operation: Box::new(Operation::TokenOperation(InnerTokenOperation {
                    token: tokens[2].clone(),
                })),
            }
        });

        return b;
    }

    if tokens.len() == 4 && tokens[3].token_type == TokenType::Assignment {
        let b = Expression::Assignment({
            InnerAssignment {
                expression_1: Box::new(Expression::TokenExpression(InnerTokenExpression {
                    token: tokens[0].clone(),
                })),

                expression_2: Box::new(Expression::TokenExpression(InnerTokenExpression {
                    token: tokens[1].clone(),
                })),

                expression_3: Box::new(Expression::TokenExpression(InnerTokenExpression {
                    token: tokens[2].clone(),
                })),

                operation: Box::new(Operation::TokenOperation(InnerTokenOperation {
                    token: tokens[3].clone(),
                })),
            }
        });

        return b;
    }

    return Expression::None;
}

pub fn parse(exp: Expression) -> Expression {
    match exp {
        Expression::ExpExpOp(i) => {
            return run_operation(i.expression_1, i.expression_2, i.operation)
        }
        Expression::Assignment(a) => {
            return create_assignment(a.expression_1, a.expression_2, a.expression_3, a.operation)
        }
        _ => return Expression::None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_operation_test() {
        let b = Expression::ExpExpOp({
            InnerExpExpOp {
                expression_1: Box::new(Expression::TokenExpression(InnerTokenExpression {
                    token: Token {
                        value: "1".to_string(),
                        token_type: TokenType::NumericIntLiteral,
                    },
                })),

                expression_2: Box::new(Expression::TokenExpression(InnerTokenExpression {
                    token: Token {
                        value: "2".to_string(),
                        token_type: TokenType::NumericIntLiteral,
                    },
                })),

                operation: Box::new(Operation::TokenOperation(InnerTokenOperation {
                    token: Token {
                        value: "+".to_string(),
                        token_type: TokenType::Addition,
                    },
                })),
            }
        });

        // b is basically "1 2 +" as an expression

        let output = parse(b);

        // output is "3"

        match output {
            Expression::TokenExpression(d) => {
                assert_eq!(d.token.token_type, TokenType::NumericIntLiteral);
                assert_eq!(d.token.value, String::from("3"));
            }

            _ => assert!(false),
        }

        let c = Expression::ExpExpOp({
            InnerExpExpOp {
                expression_1: Box::new(Expression::TokenExpression(InnerTokenExpression {
                    token: Token {
                        value: "8".to_string(),
                        token_type: TokenType::NumericIntLiteral,
                    },
                })),

                expression_2: Box::new(Expression::TokenExpression(InnerTokenExpression {
                    token: Token {
                        value: "10".to_string(),
                        token_type: TokenType::NumericIntLiteral,
                    },
                })),

                operation: Box::new(Operation::TokenOperation(InnerTokenOperation {
                    token: Token {
                        value: "*".to_string(),
                        token_type: TokenType::Multiplication,
                    },
                })),
            }
        });

        // c is basically "8 10 *" as an expression

        let output = parse(c);

        // output is "80"

        match output {
            Expression::TokenExpression(e) => {
                assert_eq!(e.token.token_type, TokenType::NumericIntLiteral);
                assert_eq!(e.token.value, String::from("80"));
            }

            _ => assert!(false),
        }
    }

    #[test]
    fn create_assignment_test() {
        let b = Expression::Assignment({
            InnerAssignment {
                expression_1: Box::new(Expression::TokenExpression(InnerTokenExpression {
                    token: Token {
                        value: "foo".to_string(),
                        token_type: TokenType::Identifier,
                    },
                })),

                expression_2: Box::new(Expression::TokenExpression(InnerTokenExpression {
                    token: Token {
                        value: "int".to_string(),
                        token_type: TokenType::TypeIntKeyword,
                    },
                })),

                expression_3: Box::new(Expression::TokenExpression(InnerTokenExpression {
                    token: Token {
                        value: "101".to_string(),
                        token_type: TokenType::NumericIntLiteral,
                    },
                })),

                operation: Box::new(Operation::TokenOperation(InnerTokenOperation {
                    token: Token {
                        value: "=".to_string(),
                        token_type: TokenType::Assignment,
                    },
                })),
            }
        });

        // b is basically "foo int 101 =" as an expression

        let output = parse(b);

        match output {
            Expression::TokenExpression(d) => {
                assert_eq!(d.token.token_type, TokenType::NumericIntLiteral);
                assert_eq!(d.token.value, String::from("101"));
            }

            _ => assert!(false),
        }
    }

    #[test]
    fn parser_one_test() {
        let mut lex: Lexer = Lexer::new(vec!["a int 3 =".to_string()]);
        let mut tokens: Vec<Token> = vec![];

        let mut tok = lex.next();
        while tok.token_type != TokenType::EndToken {
            tokens.push(tok.clone());
            tok = lex.next();
        }

        let expr = tokens_to_expr(tokens);

        let output = parse(expr);

        match output {
            Expression::TokenExpression(d) => {
                assert_eq!(d.token.token_type, TokenType::NumericIntLiteral);
                assert_eq!(d.token.value, String::from("3"));
            }

            _ => assert!(false),
        }
    }

    #[test]
    fn parser_two_test() {
        let mut lex: Lexer = Lexer::new(vec!["foo float 39400 =".to_string()]);
        let mut tokens: Vec<Token> = vec![];

        let mut tok = lex.next();
        while tok.token_type != TokenType::EndToken {
            tokens.push(tok.clone());
            tok = lex.next();
        }

        let expr = tokens_to_expr(tokens);

        let output = parse(expr);

        match output {
            Expression::TokenExpression(d) => {
                assert_eq!(d.token.token_type, TokenType::NumericIntLiteral);
                assert_eq!(d.token.value, String::from("39400"));
            }

            _ => assert!(false),
        }
    }
}
