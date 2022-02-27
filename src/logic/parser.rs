use super::Token;

pub fn parse(formula: &str) -> Vec<Token> {
    formula.split_whitespace().map(Token::new).collect()
}
