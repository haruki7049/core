pub mod parser;

use crate::parser::{Literal, Token};
use std::path::PathBuf;

#[derive(Debug)]
pub struct CoreAST {
    function: Token,
    name: Token,
    value: Token,
}

pub fn eval(config_path: &PathBuf) -> Result<Vec<CoreAST>, Box<dyn std::error::Error>> {
    let context: String = std::fs::read_to_string(config_path).unwrap_or_default();
    let tokens: Vec<Token> = parser::parse(&context)?;
    let mut result: Vec<CoreAST> = Vec::new();

    for t in tokens {
        match t {
            Token::SExpression(tokens) => result.push(eval_sexpr(tokens)?),
            _ => todo!(),
        }
    }

    Ok(result)
}

fn eval_sexpr(mut tokens: Vec<Token>) -> Result<CoreAST, Box<dyn std::error::Error>> {
    // Take the first argument
    tokens.reverse();
    let function: Token = tokens.pop().ok_or("EVAL_ERROR: Failed to read function")?;

    // Re-reverse to fix the order
    tokens.reverse();

    match function {
        Token::Literal(Literal::Define) => {
            tokens.reverse();
            let name_t: Token = tokens
                .pop()
                .ok_or("EVAL_ERROR: No name to define a variable")?;
            let value_t: Token = tokens
                .pop()
                .ok_or("EVAL_ERROR: No value to define a variable")?;

            Ok(CoreAST {
                function: Token::Literal(Literal::Define),
                name: name_t,
                value: value_t,
            })
        }
        _ => todo!(),
    }
}
