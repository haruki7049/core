use core_lang::CoreAST;
use directories::ProjectDirs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: Config = Config::load()?;
    dbg!(&config);

    Ok(())
}

#[derive(Debug)]
struct Config {
    path: PathBuf,
    context: Vec<CoreAST>,
}

impl Config {
    fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let proj_dirs = ProjectDirs::from("dev", "haruki7049", "Core")
            .ok_or("CONFIG_LOAD_ERROR: Failed to create Project Directories")?;

        let config_path: PathBuf = PathBuf::from(proj_dirs.config_dir()).join("init.core");
        let config_context: Vec<CoreAST> = core_lang::eval(&config_path)?;

        Ok(Config {
            path: config_path,
            context: config_context,
        })
    }
}
