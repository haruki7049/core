use core_lang::ast::AST;
use core_lang::evaluator;
use directories::ProjectDirs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ast = load_config()?;
    let path = core_cli::cli(&ast)?;

    dbg!(path);

    Ok(())
}

fn load_config() -> Result<AST, Box<dyn std::error::Error>> {
    let proj_dirs = ProjectDirs::from("dev", "haruki7049", "Core")
        .ok_or("CONFIG_LOAD_ERROR: Failed to create Project Directories")?;

    let config_path: PathBuf = PathBuf::from(proj_dirs.config_dir()).join("init.core");
    let config: String = std::fs::read_to_string(&config_path).unwrap_or_default();
    let result: AST = evaluator::eval(&config)?;

    Ok(result)
}
