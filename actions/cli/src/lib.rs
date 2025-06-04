use argparse::{ArgumentParser, Store};
use core_lang::ast::AST;
use core_lang::ast::Boolean;
use core_lang::ast::BuiltinWord;
use core_lang::ast::Constant;
use core_lang::ast::Value;
use std::path::PathBuf;

pub fn cli(ast: &AST) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let constants: Vec<Constant> = ast.0.clone();
    let cli_options: CLIOption = judge_cli_option(constants)?;

    let mut path: PathBuf = PathBuf::new();

    if cli_options.enable {
        let mut parser = ArgumentParser::new();
        parser.set_description(env!("CARGO_PKG_DESCRIPTION"));
        parser.refer(&mut path).add_argument("FILENAME", Store, "");
        parser.parse_args_or_exit();
    }

    Ok(path)
}

fn judge_cli_option(constants: Vec<Constant>) -> Result<CLIOption, Box<dyn std::error::Error>> {
    let mut result = CLIOption::default();

    for constant in constants {
        if constant.name() == Value::BuiltinWord(BuiltinWord::Cli) {
            let (_args, expr): (Vec<Value>, Vec<Value>) = match constant.value() {
                Value::SExpression(values) => match &values[0] {
                    Value::Lambda((args, expr)) => (args.clone(), expr.clone()),
                    _ => panic!(),
                },
                _ => panic!(),
            };

            for value in expr {
                let function = value.car()?;

                match function {
                    Value::Word(v) => match v.as_str() {
                        "enable" => {
                            let mut v: Vec<Value> = value.cdr()?;
                            result.enable = read_boolean(&v.pop().unwrap())?;

                            if v.pop().is_some() {
                                panic!();
                            }
                        }
                        _ => (),
                    },
                    _ => panic!(),
                }
            }
        }
    }

    Ok(result)
}

fn read_boolean(value: &Value) -> Result<bool, Box<dyn std::error::Error>> {
    match value {
        Value::Boolean(v) => match v {
            Boolean::T => Ok(true),
            Boolean::Nil => Ok(false),
        },
        _ => panic!(),
    }
}

#[derive(Debug, Default)]
struct CLIOption {
    enable: bool,
}
