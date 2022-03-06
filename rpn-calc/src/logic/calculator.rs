use super::Token;
use anyhow::{ensure, Context, Result};
use log::info;

pub fn calc(tokens: &[Token]) -> Result<i64> {
    let mut stack = Vec::<i64>::new();
    for token in tokens {
        info!("Stack: {:?}, Token: {:?}", stack, token);

        if let Token::Number(x) = *token {
            stack.push(x);
        } else {
            let x = stack.pop().context("Too few arguments")?;
            let y = stack.pop().context("Too few arguments")?;
            let z = match *token {
                Token::Plus => x + y,
                Token::Minus => x - y,
                Token::Multiply => x * y,
                Token::Number(_) => unreachable!(),
            };
            stack.push(z);
        }
    }

    info!("Stack: {:?}", stack);
    ensure!(stack.len() <= 1, "Too many arguments");
    stack.pop().context("Too few arguments")
}
