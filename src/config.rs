use std::{env, fs, io::Error};

use dirs::{config_dir, home_dir};
use serde::Deserialize;
use toml::{value, Value};

mod dotfile;

#[derive(Debug)]
pub struct Config {
    pub dir: String,
    pub dotfiles: Vec<dotfile::Dotfile>,
}

pub fn load_config() -> Result<Config, Error> {
    #[derive(Deserialize)]
    struct UserConfig {
        dir: String,
        dotfiles: value::Array,
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

    let dir = expand_home(user_config.dir);

    let mut dotfiles = Vec::new();
    for df in user_config.dotfiles {
        let path = get_item("path", &df, &Value::from(""))
            .to_string()
            .trim_matches('"')
            .to_string();
        let target = get_item("target", &df, &Value::from(""))
            .to_string()
            .trim_matches('"')
            .to_string();
        let sync_type = match get_item("type", &df, &Value::from("symlink")).as_str() {
            Some("symlink") => dotfile::SyncType::SymLink,
            Some("hardlink") => dotfile::SyncType::HardLink,
            Some("junction") => dotfile::SyncType::Junction,
            Some("copy") => dotfile::SyncType::Copy,
            _ => dotfile::SyncType::SymLink,
        };

        dotfiles.push(dotfile::Dotfile::new(
            format!("{}/{}", &dir, path).replace("/", "\\"),
            expand_home(target),
            sync_type,
        ));
    }

    Ok(Config { dir, dotfiles })
}

fn get_item<'a>(item_name: &'a str, val: &'a Value, default: &'a Value) -> &'a Value {
    match val.get(item_name) {
        Some(item) => match item {
            Value::Table(t) => match t.get(env::consts::OS) {
                Some(val) => val,
                None => t.get("default").unwrap(),
            },
            _ => item,
        },
        None => default,
    }
}

fn expand_home(path: String) -> String {
    if path.starts_with('~') {
        let mut p = home_dir().unwrap();
        let home: &[_] = &['~', '/', '\\'];
        p.push(path.trim_start_matches(home).to_string());
        p.display().to_string()
    } else {
        path
    }
}
