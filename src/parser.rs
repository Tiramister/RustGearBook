use std::fmt;

#[derive(Debug)]
pub enum Token {
    Plus,
    Minus,
    Multiply,
    Number(i64),
}

impl Token {
    fn new(symbol: &str) -> Token {
        match symbol {
            "+" => Token::Plus,
            "-" => Token::Minus,
            "*" => Token::Multiply,
            _ => Token::Number(
                symbol
                    .parse::<i64>()
                    .unwrap_or_else(|_| panic!("Invalid token: {}", symbol)),
            ),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Multiply => write!(f, "*"),
            Self::Number(n) => write!(f, "{}", n),
        }
    }
}

pub fn parse(formula: &str) -> Vec<Token> {
    formula.split_whitespace().map(Token::new).collect()
}
