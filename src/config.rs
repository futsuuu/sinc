use std::{fs, io::Error, path::PathBuf};

use serde::Deserialize;
use toml::{value, Value};

#[derive(Debug)]
pub struct Config {
    pub dotfiles: Vec<Dotfile>,
}

#[derive(Debug)]
pub struct Dotfile {
    pub dir: String,
    pub path: String,
    pub target: String,
    pub sync_type: String,
}

pub fn load_config(config_path: String) -> Result<Config, Error> {
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
        let s = fs::read_to_string(PathBuf::from(config_path))?;
        toml::from_str(&s).unwrap()
    };

    let mut dotfiles = Vec::new();
    for df in user_config.dotfiles {
        let dir = match df.get("dir") {
            Some(t) => t,
            None => &user_config.default.dir,
        };
        let sync_type = match df.get("sync_type") {
            Some(t) => t,
            None => &user_config.default.sync_type,
        };
        let path = df.get("path").unwrap();
        let target = df.get("target").unwrap();

        dotfiles.push(Dotfile {
            dir: val2string(dir),
            path: val2string(path),
            target: val2string(target),
            sync_type: val2string(sync_type),
        });
    }

    Ok(Config { dotfiles })
}

fn val2string(s: &Value) -> String {
    s.to_string().trim_matches('"').to_string()
}
