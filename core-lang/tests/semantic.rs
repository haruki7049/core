use core_lang::ast::AST;
use core_lang::evaluator;
use std::path::PathBuf;

#[test]
fn test() -> Result<(), Box<dyn std::error::Error>> {
    for entry in std::fs::read_dir("./tests/semantic-files")? {
        let path: PathBuf = entry?.path();
        let data: String = std::fs::read_to_string(path).unwrap_or_default();

        let result: AST = evaluator::eval(&data)?;
    }

    Ok(())
}
