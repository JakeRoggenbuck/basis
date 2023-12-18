use crate::lexer::{Lex, Lexer, Token, TokenType};

enum ParserBlock {
    Literal(Token),
    Op(Token),
}

#[cfg(test)]
mod tests {
    use super::*;

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
