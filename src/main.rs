use std::{env, io::Error};

use dirs::home_dir;

mod config;
mod dotfile;

fn main() -> Result<(), Error> {
    let config_file = format!(
        "{}/sing/sing.toml",
        match env::var("XDG_CONFIG_HOME") {
            Ok(p) => p,
            Err(_) => "~/.config".to_string(),
        }
    );
    let config_data = config::load_config(correct_path(config_file))?;

    let mut dotfiles = Vec::new();

    for df in config_data.dotfiles {
        dotfiles.push(dotfile::Dotfile::new(
            correct_path(format!("{}/{}", df.dir, df.path)),
            correct_path(df.target),
            df.sync_type,
        ));
    }

    for df in dotfiles {
        df.sync()?;
    }

    Ok(())
}

fn correct_path(path: String) -> String {
    struct Separator<'a>(&'a str, &'a str);
    let sep = if cfg!(target_os = "windows") {
        Separator("/", "\\")
    } else {
        Separator("\\", "/")
    };
    if path.starts_with('~') {
        let mut p = home_dir().unwrap();
        let home: &[_] = &['~', '/', '\\'];
        p.push(path.trim_start_matches(home));
        p.display().to_string()
    } else {
        path
    }
    .replace(sep.0, sep.1)
}
