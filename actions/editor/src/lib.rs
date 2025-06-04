use core_lang::ast::AST;
use std::path::PathBuf;

pub fn editor(ast: AST, path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    dbg!(ast);
    dbg!(path);
    todo!()
}
