use crate::lexer::{Lex, Lexer, Token, TokenType};

#[derive(Debug)]
pub struct InnerTokenExpression {
    token: Token,
}

/// Expression Expression Operation
#[derive(Debug)]
pub struct InnerExpExpOp {
    expression_1: Box<Expression>,
    expression_2: Box<Expression>,
    operation: Box<Operation>,
}

#[derive(Debug)]
pub enum Expression {
    TokenExpression(InnerTokenExpression),
    ExpExpOp(InnerExpExpOp),
    None,
}

#[derive(Debug)]
pub struct InnerTokenOperation {
    token: Token,
}

#[derive(Debug)]
pub enum Operation {
    TokenOperation(InnerTokenOperation),
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

    return Expression::None;
}

pub fn parse(exp: Expression) -> Expression {
    match exp {
        Expression::ExpExpOp(i) => {
            return run_operation(i.expression_1, i.expression_2, i.operation)
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
    }

    #[test]
    fn parser_test() {
        let mut lex: Lexer = Lexer::new(vec!["a int 3 =".to_string()]);
        let mut tokens: Vec<Token> = vec![];

        let mut tok = lex.next();
        while tok.token_type == TokenType::EndToken {
            tokens.push(tok.clone());
            tok = lex.next();
        }
    }
}
