use itertools::Itertools;

fn is_type(maybe_type: String) -> bool {
    match maybe_type.as_str() {
        // Number types:
        "number" | "int" | "ratio" | "real" | "dec" | "complex" | "imaginary" | "size"
        // Other types
        | "literal" | "type" | "option" | "string" => true,
        _ => false,
    }
}

fn is_char_symbol(ch: char) -> bool {
    match ch {
        '[' | ']' | '{' | '}' | '(' | ')' | '.' | ',' | ':' | ';' | '=' | '\'' | '\"' | '\\'
        | '+' | '-' | '*' | '/' | '^' | '>' | '<' | '~' => true,
        _ => false,
    }
}

fn is_non_zero_number(ch: char) -> bool {
    match ch {
        '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => true,
        _ => false,
    }
}

fn is_number(ch: char) -> bool {
    ch == '0' || is_non_zero_number(ch)
}

fn is_char_whitespace(ch: char) -> bool {
    match ch {
        '\t' | ' ' | '\n' => true,
        _ => false,
    }
}

fn ends_token(cur: char, next: char) -> bool {
    if is_char_whitespace(next) {
        return true;
    }

    if is_char_symbol(cur) || is_char_symbol(next) {
        return true;
    }

    if is_char_whitespace(cur) {}
    return false;
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    NoType = 0,

    // 100, 5, 34298, -43
    NumericIntLiteral = 1,

    // 100.10, 0.0124, 45.453
    // NOT: .3, 54
    NumericDecLiteral = 2,

    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    LeftParen,
    RightParen,

    Dot,
    Comma,

    Assignment,
    Semicolon,
    Colon,
    Tag,
    Reference,
    Question,
    At,
    Percent,
    Bang,
    BackSlash,

    Space,
    Tab,
    Newline,

    SingleQuote,
    DoubleQuote,
}

pub trait TokenTrait {
    fn default() -> Self;
    fn tokenize(tokens: String) -> Self;
    fn from_chars(chars: Vec<char>) -> Self;
}

pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

fn is_part_int_numeric(part: &str) -> bool {
    let mut chars = part.chars();

    let first_char = chars
        .nth(0)
        .expect("Part should have more than zero characters");
    if !(is_non_zero_number(first_char) || first_char == '-') {
        return false;
    }

    for c in chars {
        if !is_number(c) {
            return false;
        }
    }

    return true;
}

impl TokenTrait for Token {
    fn default() -> Self {
        return Token {
            token_type: TokenType::NoType,
            value: String::new(),
        };
    }

    fn tokenize(tokens: String) -> Self {
        let token_str = tokens.as_str();
        let mut token = Token::default();

        if tokens.len() == 1 {
            let token_type = match token_str {
                "{" => TokenType::LeftBrace,
                "}" => TokenType::RightBrace,
                "[" => TokenType::LeftBracket,
                "]" => TokenType::RightBracket,
                "(" => TokenType::LeftParen,
                ")" => TokenType::RightParen,

                "." => TokenType::Dot,
                "," => TokenType::Comma,

                "=" => TokenType::Assignment,
                ";" => TokenType::Semicolon,
                ":" => TokenType::Colon,
                "#" => TokenType::Tag,
                "&" => TokenType::Reference,
                "?" => TokenType::Question,
                "@" => TokenType::At,
                "%" => TokenType::Percent,
                "!" => TokenType::Bang,
                "\\" => TokenType::BackSlash,

                " " => TokenType::Space,
                "\t" => TokenType::Tab,
                "\n" => TokenType::Newline,

                "\'" => TokenType::SingleQuote,
                "\"" => TokenType::DoubleQuote,

                _ => TokenType::NoType,
            };

            if token_type != TokenType::NoType {
                token.token_type = token_type;
                token.value = tokens;
                return token;
            }
        }

        if is_part_int_numeric(token_str) {
            token.token_type = TokenType::NumericIntLiteral;
            token.value = tokens;
            return token;
        }

        token.value = tokens;
        return token;
    }

    fn from_chars(chars: Vec<char>) -> Self {
        let string: String = String::from_iter(chars);
        Token::tokenize(string)
    }
}

pub struct Lexer {
    line_index: usize,
    column_index: usize,
    lines: Vec<String>,
}

pub trait Lex {
    fn new() -> Self;
    fn next(&mut self) -> Token;
}

impl Lex for Lexer {
    fn new() -> Self {
        Lexer {
            line_index: 0,
            column_index: 0,
            lines: vec![],
        }
    }

    fn next(&mut self) -> Token {
        let mut buffer = Vec::<char>::new();

        // Note: the last character in the line will never be set to `cur`, thus will never get
        // pushed. You could push(' ') to the line to fix this as seen below. The line is mutable
        // anyway so there isn't much of a real disadvantage do doing this
        self.lines[self.line_index].push(' ');

        let current_line = &self.lines[self.line_index][self.column_index..];

        for (cur, next) in current_line.chars().into_iter().tuple_windows() {
            self.column_index += 1;
            buffer.push(cur);
            if ends_token(cur, next) {
                self.column_index += 1;
                break;
            }
        }

        return Token::from_chars(buffer);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_default_test() {
        assert_eq!(Token::default().token_type, TokenType::NoType);
    }

    #[test]
    fn tokenize_test() {
        assert_eq!(
            Token::tokenize("1".to_string()).token_type,
            TokenType::NumericIntLiteral
        );

        assert_eq!(Token::tokenize("1".to_string()).value, "1".to_string());

        // Note: "1 " should NOT be a valid token, because the value given to tokenize should cut
        // off after the 1 because of the ending token check with ends_token
        // However, when this is given to the lexer, it should lex "1 " as a valid
        // NumericIntLiteral type with a value of "1"
        assert_ne!(
            Token::tokenize("1 ".to_string()).token_type,
            TokenType::NumericIntLiteral
        );
        assert_ne!(Token::tokenize("1 ".to_string()).value, "1".to_string());
    }

    #[test]
    fn lexer_test() {
        let mut lex: Lexer = Lexer::new();

        lex.lines = vec!["a int 5 =".to_string()];

        assert_eq!(lex.next().value, "a");
        assert_eq!(lex.next().value, "int");
        assert_eq!(lex.next().value, "5");
        assert_eq!(lex.next().value, "=");
    }

    #[test]
    fn is_type_test() {
        assert!(is_type("dec".to_string()));
    }

    #[test]
    fn is_char_symbol_test() {
        assert!(is_char_symbol('+'));
    }

    #[test]
    fn is_char_whitespace_test() {
        assert!(is_char_whitespace(' '));
    }

    #[test]
    fn ends_token_test() {
        // "1 " is the `1` token
        assert!(ends_token('1', ' '));

        // "+\n" is the `+` token
        assert!(ends_token('+', '\n'));

        // "myvar " is the `literal` token containing "myvar"
        assert!(ends_token('r', ' '));

        // "+=" is the `sum` token
        assert!(ends_token('+', '='));

        // "ab" is not the end of any token and may continue as a literal
        assert!(!ends_token('a', 'b'));
    }
}
