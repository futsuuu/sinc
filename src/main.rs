use std::{env, io::Error};

mod config;

fn main() -> Result<(), Error> {
    let config_file = format!(
        "{}/sing/sing.toml",
        match env::var("XDG_CONFIG_HOME") {
            Ok(p) => p,
            Err(_) => "~/.config".to_string(),
        }
    );
    let config_data = config::load_config(config_file)?;

    for dotfile in config_data.dotfiles {
        dotfile.sync()?;
    }

    Ok(())
}
