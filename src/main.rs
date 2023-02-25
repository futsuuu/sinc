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
            path::to_correct(format!("{}/{}", df.dir, df.path)),
            path::to_correct(df.target),
            df.sync_type,
            df.enable,
        ));
    }

    let mut progress = ui::Progress::new(*&dotfiles.len() as u16);

    for df in dotfiles {
        progress.message = df.get_message();
        progress.draw()?;
        df.sync()?;
        progress.val += 1;
    }

    progress.message = format!("success");
    progress.end()?;

    Ok(())
}
