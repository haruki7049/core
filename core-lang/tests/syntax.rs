use core_lang::parser;
use core_lang::token::Token;
use std::path::PathBuf;

#[test]
fn test() -> Result<(), Box<dyn std::error::Error>> {
    for entry in std::fs::read_dir("./tests/syntax-files")? {
        let path: PathBuf = entry?.path();
        let data: String = std::fs::read_to_string(path).unwrap_or_default();

        let _result: Vec<Token> = parser::parse(&data)?;
    }

    Ok(())
}
