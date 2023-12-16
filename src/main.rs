use crate::lexer::{Lex, Lexer, TokenType};
use efcl::{bold, color, Color};
use std::io::{stdin, stdout, Write};

pub mod lexer;

fn interactive() {
    loop {
        print!("{}", color!(Color::GREEN, "\n> "));
        stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();

        let _ = stdin().read_line(&mut input);

        let mut lex: Lexer = Lexer::new(vec![input]);

        loop {
            let a = lex.next();

            if a.token_type == TokenType::EndToken {
                break;
            }

            println!("{:?}", a);
        }
    }
}

fn main() {
    interactive();
}
