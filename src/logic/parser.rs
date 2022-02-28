use super::Token;
use anyhow::Result;

pub fn parse(formula: &str) -> Result<Vec<Token>> {
    formula.split_whitespace().map(Token::new).collect()
}
