use clap::{Command, arg};
use core_lang::ast::AST;
use core_lang::ast::Boolean;
use core_lang::ast::BuiltinWord;
use core_lang::ast::Constant;
use core_lang::ast::Value;

pub fn cli(ast: &AST) -> Result<(), Box<dyn std::error::Error>> {
    let constants: Vec<Constant> = ast.0.clone();
    let cli_options: CLIOption = judge_cli_option(constants)?;

    if cli_options.use_clap {
        let matches = Command::new("core")
            .version(env!("CARGO_PKG_VERSION"))
            .arg(arg!(<PATH> "FILEPATH"))
            .get_matches();

        dbg!(matches);
    }

    Ok(())
}

fn judge_cli_option(constants: Vec<Constant>) -> Result<CLIOption, Box<dyn std::error::Error>> {
    let mut result = CLIOption::default();

    for constant in constants {
        if constant.name() == Value::BuiltinWord(BuiltinWord::Cli) {
            let list: Vec<Value> = match constant.value() {
                Value::List(v) => v,
                _ => panic!(),
            };

            for value in list {
                let function = value.car()?;

                match function {
                    Value::Word(v) => match v.as_str() {
                        "use-clap" => {
                            let mut v: Vec<Value> = value.cdr()?;
                            result.use_clap = read_boolean(&v.pop().unwrap())?;

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
    use_clap: bool,
}
