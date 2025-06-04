use std::path::PathBuf;
use core_lang::ast::AST;

pub fn editor(ast: AST, path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    dbg!(ast);
    dbg!(path);
    todo!()
}
