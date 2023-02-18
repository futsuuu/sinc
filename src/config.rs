use std::{env, fs, io::Error};

use dirs::config_dir;
use serde::Deserialize;
use toml::{value, Value};

#[derive(Deserialize)]
struct UserConfig {
    dotfiles: value::Array,
}

#[derive(Debug)]
pub struct Config {
    pub dotfiles: Vec<Dotfile>,
}

#[derive(Debug)]
pub struct Dotfile {
    pub config: String,
    pub path: String,
    pub link: bool,
}

pub fn load_config() -> Result<Config, Error> {
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
    for dotfile in user_config.dotfiles {
        let config = get_item("config", &dotfile, &Value::from(""))
            .to_string()
            .trim_matches('"')
            .to_string();
        let path = get_item("path", &dotfile, &Value::from("~"))
            .to_string()
            .trim_matches('"')
            .to_string();
        let link = get_item("link", &dotfile, &Value::from(true))
            .as_bool()
            .unwrap();

        dotfiles.push(Dotfile { config, path, link })
    }

    Ok(Config { dotfiles })
}

fn get_item<'a>(item_name: &'a str, dotfile: &'a Value, default: &'a Value) -> &'a Value {
    match dotfile.get(item_name) {
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
