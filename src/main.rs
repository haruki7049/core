use clap::Parser;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _args: CLIArgs = CLIArgs::parse();
    todo!()
}

#[derive(Debug, Parser)]
struct CLIArgs {
    filepath: PathBuf,
}
