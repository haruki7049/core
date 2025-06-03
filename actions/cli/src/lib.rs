use clap::Command;
use core_lang::ast::AST;
use core_lang::ast::BuiltinWord;
use core_lang::ast::Constant;
use core_lang::ast::Value;

pub fn cli(ast: &AST) -> Result<(), Box<dyn std::error::Error>> {
    if is_t(ast, "use-cli")? {
        let matches = Command::new("core");

        if is_t(ast, "show-version")? {
            matches
                .version(env!("CARGO_PKG_VERSION"))
                .get_matches();
        }
    }

    Ok(())
}

fn is_t(ast: &AST, target: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let constants: &Vec<Constant> = &ast.0;
    for constant in constants {
        match constant.name {
            Value::BuiltinWord(BuiltinWord::Cli) => {
                match &constant.value {
                    Value::List(value) => for v in value {
                        let function_name: &String = function(v)?;
                        if target == function_name {
                            return Ok(true);
                        }
                    },
                    _ => panic!(),
                }
            }
            _ => (),
        }
    }

    Ok(false)
}

fn function(value: &Value) -> Result<&String, Box<dyn std::error::Error>> {
    match value {
        Value::SExpression(v) => {
            match &v[0] {
                Value::Word(v) => Ok(v),
                _ => panic!(),
            }
        }
        _ => panic!(),
    }
}
