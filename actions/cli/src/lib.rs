use clap::Command;
use core_lang::ast::AST;
use core_lang::ast::BuiltinWord;
use core_lang::ast::Constant;
use core_lang::ast::Value;

pub fn cli(ast: &AST) -> Result<(), Box<dyn std::error::Error>> {
    let constants: Vec<Constant> = ast.0.clone();

    if is_t(&constants, "use-cli")? {
        let matches = Command::new("core");

        if is_t(&constants, "show-version")? {
            matches
                .clone()
                .version(env!("CARGO_PKG_VERSION"))
                .get_matches();
        }
    }

    Ok(())
}

fn is_t(constants: &Vec<Constant>, target: &str) -> Result<bool, Box<dyn std::error::Error>> {
    for constant in constants {
        dbg!(&constant);

        match constant.name {
            Value::BuiltinWord(BuiltinWord::Cli) => todo!(),
            _ => (),
        }
    }

    Ok(false)
}
