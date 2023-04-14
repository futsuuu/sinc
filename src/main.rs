use anyhow::Result;

mod config;
mod dotfile;
mod path;
mod ui;

fn main() -> Result<()> {
    let config_file = path::config_file();
    let config_data = config::load_config(config_file)?;

    let mut dotfiles = Vec::new();

    for df in config_data.dotfiles {
        dotfiles.push(dotfile::Dotfile::new(
            df.path.clone(),
            path::to_correct(format!("{}/{}", df.dir, df.path)),
            df.target
                .iter()
                .map(|t| path::to_correct(t.clone()))
                .collect(),
            df.sync_type,
            df.enable,
            df.hook_add,
            df.hook_read,
        ));
    }

    for df in dotfiles {
        df.sync();
    }

    println!();

    Ok(())
}
