use std::{env, fs, io::Error};

use dirs::config_dir;
use serde::Deserialize;
use toml::{value, Value};

#[derive(Deserialize)]
struct Config {
    dotfiles: value::Array,
}

#[derive(Debug)]
pub struct Data {
    pub config: String,
    pub path: String,
    pub link: bool,
}

pub fn load_config() -> Result<Vec<Data>, Error> {
    let mut data = Vec::new();
    let config: Config = {
        let s = fs::read_to_string(
            config_dir()
                .unwrap()
                .join("sing/config.toml")
                .to_str()
                .unwrap(),
        )?;

        toml::from_str(&s).unwrap()
    };

    for dotfile in config.dotfiles {
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

        data.push(Data { config, path, link })
    }

    Ok(data)
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
