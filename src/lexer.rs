#[derive(Debug, PartialEq)]
pub enum TokenType {
    NoType = 0,
}

pub trait TokenTrait {
    fn default() -> Self;
}

pub struct Token {
    pub token_type: TokenType,
}

impl TokenTrait for Token {
    fn default() -> Self {
        return Token {
            token_type: TokenType::NoType,
        };
    }
}

pub struct Lexer {}

pub trait Lex {
    fn new() -> Self;
    fn next(&mut self) -> Token;
}

impl Lex for Lexer {
    fn new() -> Self {
        Lexer {}
    }

    fn next(&mut self) -> Token {
        Token {
            token_type: TokenType::NoType,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_default_test() {
        assert_eq!(Token::default().token_type, TokenType::NoType);
    }
}
