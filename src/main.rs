use std::io::Error;

mod config;
mod dotfile;
mod path;
mod ui;

fn main() -> Result<(), Error> {
    let config_file = path::config_file();
    let config_data = config::load_config(config_file)?;

    let mut dotfiles = Vec::new();

    for df in config_data.dotfiles {
        dotfiles.push(dotfile::Dotfile::new(
            df.path.clone(),
            path::to_correct(format!("{}/{}", df.dir, df.path)),
            path::to_correct(df.target),
            df.sync_type,
            df.enable,
        ));
    }

    for df in dotfiles {
        df.sync()?;
        println!("{}", df.get_message());
    }

    println!();

    Ok(())
}
