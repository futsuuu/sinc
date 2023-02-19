use std::{fs, io::Error, path::PathBuf};

mod config;

fn main() -> Result<(), Error> {
    let config_data = config::load_config()?;
    let dotfiles_dir = PathBuf::from(&config_data.dir);
    if !dotfiles_dir.exists() {
        fs::create_dir_all(&dotfiles_dir)?;
    }

    for dotfile in config_data.dotfiles {
        dotfile.sync()?;
    }

    Ok(())
}
