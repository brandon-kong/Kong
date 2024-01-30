#[derive(Debug, PartialEq)]

pub enum Token {
    Integer(i32),
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    OpenParen,
    CloseParen,
}

pub(crate) fn lexer(input: &str) -> Vec<Token> {
    let mut result = Vec::new();

    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            '+' => {
                chars.next();
                result.push(Token::Plus);
            }
            '-' => {
                chars.next();
                result.push(Token::Minus);
            }
            '*' => {
                chars.next();
                result.push(Token::Multiply);
            }
            '/' => {
                chars.next();
                result.push(Token::Divide);
            }
            '^' => {
                chars.next();
                result.push(Token::Power);
            }
            '(' => {
                chars.next();
                result.push(Token::OpenParen);
            }
            ')' => {
                chars.next();
                result.push(Token::CloseParen);
            }
            '0'..='9' => {
                let mut number = 0;
                while let Some('0'..='9') = chars.peek() {
                    number = number * 10 + chars.next().unwrap().to_digit(10).unwrap() as i32;
                }
                result.push(Token::Integer(number));
            }
            _ => {
                chars.next();
            }
        }
    }

    result
}