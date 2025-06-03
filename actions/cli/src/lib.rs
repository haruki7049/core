use clap::Command;
use core_lang::ast::AST;
use core_lang::ast::BuiltinWord;
use core_lang::ast::Constant;
use core_lang::ast::Value;

pub fn cli(ast: &AST) -> Result<(), Box<dyn std::error::Error>> {
    let help: bool = is_t(ast, "help")?;

    let matches = Command::new("core")
        .version(env!("CARGO_PKG_VERSION"))
        .get_matches();

    Ok(())
}

fn is_t(ast: &AST, target: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let constants: &Vec<Constant> = &ast.0;
    for constant in constants {
        match constant.name {
            Value::BuiltinWord(BuiltinWord::Cli) => {
                dbg!(&constant.value);
            }
            _ => (),
        }
    }

    Ok(true)
}
