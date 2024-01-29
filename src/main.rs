use crate::lexer::{Lex, Lexer, Token, TokenType};
use crate::parser::{parse, tokens_to_expr, Expression};
use efcl::{bold, color, Color};
use std::io::{stdin, stdout, Write};

pub mod lexer;
pub mod parser;

fn interactive() {
    loop {
        print!("{}", color!(Color::GREEN, "\n> "));
        stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();

        let _ = stdin().read_line(&mut input);

        let mut lex: Lexer = Lexer::new(vec![input]);

        let mut tokens = Vec::<Token>::new();

        loop {
            let a = lex.next();

            if a.token_type == TokenType::EndToken {
                break;
            }

            println!("{:?}", a);
            tokens.push(a);
        }

        let expr = tokens_to_expr(tokens);
        let out = parse(expr);

        match out {
            Expression::None => {}
            _ => {
                println!("-> {:?}", out);
            }
        }
    }
}

fn main() {
    interactive();
}
