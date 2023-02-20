use std::{fs, io::Error};

use dirs::{config_dir, home_dir};
use serde::Deserialize;
use toml::{value, Value};

mod dotfile;

#[derive(Debug)]
pub struct Config {
    pub dotfiles: Vec<dotfile::Dotfile>,
}

pub fn load_config() -> Result<Config, Error> {
    #[derive(Deserialize)]
    struct UserConfig {
        default: DefaultVal,
        dotfiles: value::Array,
    }

    #[derive(Deserialize)]
    struct DefaultVal {
        dir: Value,
        sync_type: Value,
    }

    let user_config: UserConfig = {
        let s = fs::read_to_string(
            config_dir()
                .unwrap()
                .join("sing/config.toml")
                .to_str()
                .unwrap(),
        )?;

        toml::from_str(&s).unwrap()
    };

    let mut dotfiles = Vec::new();
    for df in user_config.dotfiles {
        let dir = {
            match df.get("dir") {
                Some(t) => t,
                None => &user_config.default.dir,
            }
            .to_string()
            .trim_matches('"')
            .to_string()
        };
        let sync_type = {
            match df.get("sync_type") {
                Some(t) => t,
                None => &user_config.default.sync_type,
            }
            .as_str()
        };
        let path = df
            .get("path")
            .unwrap()
            .to_string()
            .trim_matches('"')
            .to_string();
        let target = df
            .get("target")
            .unwrap()
            .to_string()
            .trim_matches('"')
            .to_string();

        dotfiles.push(dotfile::Dotfile::new(
            correct_path(format!("{}/{}", dir, path)),
            correct_path(target),
            sync_type,
        ));
    }

    Ok(Config { dotfiles })
}

fn correct_path(path: String) -> String {
    let separator = if cfg!(target_os = "windows") {
        "\\"
    } else {
        "/"
    };
    if path.starts_with('~') {
        let mut p = home_dir().unwrap();
        let home: &[_] = &['~', '/', '\\'];
        p.push(path.trim_start_matches(home).to_string());
        p.display().to_string()
    } else {
        path
    }
    .replace("/", separator)
}
