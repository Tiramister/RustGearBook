pub mod calculator;
pub mod parser;

use anyhow::{Context, Result};
use std::fmt;

#[derive(Debug)]
pub enum Token {
    Plus,
    Minus,
    Multiply,
    Number(i64),
}

impl Token {
    fn new(symbol: &str) -> Result<Token> {
        Ok(match symbol {
            "+" => Token::Plus,
            "-" => Token::Minus,
            "*" => Token::Multiply,
            _ => Token::Number(
                symbol
                    .parse::<i64>()
                    .with_context(|| format!("Invalid token: {}", symbol))?,
            ),
        })
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

pub fn calc(formula: &str) -> Result<i64> {
    calculator::calc(&parser::parse(formula)?)
}
