use log::info;

use crate::parser::Token;

pub fn calc(tokens: &[Token]) -> i64 {
    let mut stack = Vec::<i64>::new();
    for token in tokens {
        info!("Stack: {:?}, Token: {:?}", stack, token);

        if let Token::Number(x) = *token {
            stack.push(x);
        } else {
            let x = stack.pop().unwrap();
            let y = stack.pop().unwrap();
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
    stack.pop().unwrap()
}
