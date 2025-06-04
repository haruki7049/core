use crate::{
    ast::AST, ast::Boolean, ast::BuiltinWord, ast::Constant, ast::Value, parser, token::Literal,
    token::Token,
};

pub fn eval(program: &str) -> Result<AST, Box<dyn std::error::Error>> {
    let mut context: Vec<Constant> = Vec::new();
    let parser_result: Vec<Token> = parser::parse(program)?;

    for token in parser_result {
        match token {
            Token::SExpression(mut value) => {
                // Reverses the order of tokens because I want to use 'tokens.pop()'
                value.reverse();

                let function_t: Token = value.pop().ok_or("Failed to read function")?;
                match function_t {
                    Token::Literal(v) => match v {
                        Literal::Define => {
                            let name_t: Token = value.pop().ok_or("Failed to read name")?;
                            let value_t: Token = value.pop().ok_or("Failed to read value")?;

                            let result: Constant = Constant {
                                name: eval_name(name_t)?,
                                value: eval_value(value_t)?,
                            };

                            context.push(result);
                        }
                        _ => todo!(),
                    },
                    _ => todo!(),
                }
            }
            _ => unreachable!(),
        }
    }

    Ok(AST(context))
}

fn eval_name(token: Token) -> Result<Value, Box<dyn std::error::Error>> {
    match token {
        Token::Word(v) => match v.as_str() {
            "main" => Ok(Value::BuiltinWord(BuiltinWord::Main)),
            "cli" => Ok(Value::BuiltinWord(BuiltinWord::Cli)),
            "t" => Ok(Value::Boolean(Boolean::T)),
            "nil" => Ok(Value::Boolean(Boolean::Nil)),
            v => Ok(Value::Word(v.to_string())),
        },
        _ => panic!(),
    }
}

fn eval_value(token: Token) -> Result<Value, Box<dyn std::error::Error>> {
    match token {
        Token::String(v) => Ok(Value::String(v)),
        Token::Number(v) => Ok(Value::Number(v)),
        Token::List(v) => eval_list(v),
        Token::SExpression(v) => eval_sexpr(v),
        Token::Word(_) => eval_name(token),
        Token::Literal(v) => eval_literal(v),
    }
}

fn eval_literal(literal: Literal) -> Result<Value, Box<dyn std::error::Error>> {
    match literal {
        Literal::Cons => Ok(Value::BuiltinWord(BuiltinWord::Cons)),
        Literal::Car => Ok(Value::BuiltinWord(BuiltinWord::Car)),
        Literal::Cdr => Ok(Value::BuiltinWord(BuiltinWord::Cdr)),
        Literal::If => Ok(Value::BuiltinWord(BuiltinWord::If)),
        Literal::Lambda => Ok(Value::BuiltinWord(BuiltinWord::Lambda)),
        Literal::Begin => Ok(Value::BuiltinWord(BuiltinWord::Begin)),
        Literal::Define => Ok(Value::BuiltinWord(BuiltinWord::Define)),
        Literal::DefineSyntax => Ok(Value::BuiltinWord(BuiltinWord::DefineSyntax)),
        Literal::CallCc => Ok(Value::BuiltinWord(BuiltinWord::CallCc)),
    }
}

fn eval_list(tokens: Vec<Token>) -> Result<Value, Box<dyn std::error::Error>> {
    let mut result: Vec<Value> = Vec::new();

    for token in tokens {
        match token {
            Token::SExpression(v) => result.push(eval_sexpr(v)?),
            Token::List(v) => result.push(eval_list(v)?),
            Token::Word(_) => result.push(eval_name(token)?),
            Token::Literal(v) => result.push(eval_literal(v)?),
            Token::String(v) => result.push(Value::String(v)),
            Token::Number(v) => result.push(Value::Number(v)),
        }
    }

    Ok(Value::List(result))
}

fn eval_sexpr(mut tokens: Vec<Token>) -> Result<Value, Box<dyn std::error::Error>> {
    let mut result: Vec<Value> = Vec::new();

    // Reverses the order of tokens because I want to use 'tokens.pop()'
    tokens.reverse();

    let function_t: Token = tokens.pop().ok_or("Failed to read function")?;
    match function_t {
        Token::Literal(v) => match v {
            Literal::Define => {
                let name_t: Token = tokens.pop().ok_or("Failed to read name")?;
                let value_t: Token = tokens.pop().ok_or("Failed to read value")?;

                let constant: Constant = Constant {
                    name: eval_name(name_t)?,
                    value: eval_value(value_t)?,
                };

                result.push(Value::Constant(Box::new(constant)));
            }
            Literal::Lambda => {
                let args_t: Token = tokens.pop().ok_or("Failed to read arguments")?;
                let expr_t: Token = tokens.pop().ok_or("Failed to read expr")?;

                let args: Vec<Value> = vec![eval_value(args_t)?];
                let expr: Vec<Value> = vec![eval_value(expr_t)?];

                result.push(Value::Lambda((args, expr)));
            }
            _ => todo!(),
        },
        Token::Word(v) => {
            result.push(Value::Word(v));
            for token in tokens {
                result.push(eval_value(token)?);
            }
        }
        _ => todo!(),
    }

    Ok(Value::SExpression(result))
}
