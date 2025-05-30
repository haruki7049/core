use directories::ProjectDirs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Config::load()?;
    todo!();
}

struct Config;

impl Config {
    fn load() -> Result<(), Box<dyn std::error::Error>> {
        let proj_dirs = ProjectDirs::from("dev", "haruki7049", "Core")
            .ok_or("CONFIG_LOAD_ERROR: Failed to create Project Directories");
        proj_dirs?.config_dir();

        Ok(())
    }
}
