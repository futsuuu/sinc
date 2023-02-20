use std::io::Error;

mod config;

fn main() -> Result<(), Error> {
    let config_data = config::load_config()?;

    for dotfile in config_data.dotfiles {
        dotfile.sync()?;
    }

    Ok(())
}
