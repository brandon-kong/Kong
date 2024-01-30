use crate::util::tokens::lexer::Token;

pub enum Expr {
    Plus(Box<Expr>, Box<Expr>),
    Minus(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
    Integer(i32),
}


pub fn parse(tokens: Vec<Token>) -> Expr {
    let mut tokens_iter = tokens.iter().peekable();
    let expr = parse_expr(&mut tokens_iter);
    expr
}

fn parse_expr<'a, I>(tokens: &mut I) -> Expr
where
    I: Iterator<Item = &'a Token>,
{
    let mut expr = parse_term(tokens);

    while let Some(token) = tokens.next() {
        match token {
            Token::Plus => {
                let rhs = parse_term(tokens);
                expr = Expr::Plus(Box::new(expr), Box::new(rhs));
            }
            Token::Minus => {
                let rhs = parse_term(tokens);
                expr = Expr::Minus(Box::new(expr), Box::new(rhs));
            }
            Token::Multiply => {
                let rhs = parse_term(tokens);
                expr = Expr::Multiply(Box::new(expr), Box::new(rhs));
            }
            Token::Divide => {
                let rhs = parse_term(tokens);
                expr = Expr::Divide(Box::new(expr), Box::new(rhs));
            }
            
            _ => break,
        }
    }

    expr
}

fn parse_term<'a, I>(tokens: &mut I) -> Expr
where
    I: Iterator<Item = &'a Token>,
{
    match tokens.next() {
        Some(Token::Integer(n)) => Expr::Integer(*n),
        _ => panic!("unexpected token"),
    }
}

impl Expr {
    pub fn eval(&self) -> i32 {
        match self {
            Expr::Integer(n) => *n,
            Expr::Plus(lhs, rhs) => lhs.eval() + rhs.eval(),
            Expr::Minus(lhs, rhs) => lhs.eval() - rhs.eval(),
            Expr::Multiply(lhs, rhs) => lhs.eval() * rhs.eval(),
            Expr::Divide(lhs, rhs) => lhs.eval() / rhs.eval(),
        }
    }
}