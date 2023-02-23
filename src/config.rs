use std::{env::consts, fs, io::Error, path::PathBuf};

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
        let dir = get_val(&df, "dir", Some(&user_config.default.dir));
        let sync_type = get_val(&df, "sync_type", Some(&user_config.default.sync_type));
        let path = get_val(&df, "path", None);
        let target = get_val(&df, "target", None);
        dotfiles.push(Dotfile {
            dir: val2string(dir),
            path: val2string(path),
            target: val2string(target),
            sync_type: val2string(sync_type),
        });
    }

    Ok(Config { dotfiles })
}

fn val2string(s: Value) -> String {
    s.to_string().trim_matches('"').to_string()
}

fn get_val(parent_value: &Value, value_name: &str, default_value: Option<&Value>) -> Value {
    let raw_value = {
        let v = parent_value.get(value_name);
        if let Some(default_v) = default_value {
            match v {
                Some(t) => t,
                None => default_v,
            }
        } else {
            v.unwrap()
        }
    };

    fn new_arm(item_val: &Value, key_name: &str) -> Value {
        match item_val.get(key_name) {
            Some(v) => v,
            None => item_val.get("default").unwrap(),
        }
        .clone()
    }

    match raw_value {
        Value::Table(t) => {
            let mut t_iter = t.iter();
            let child = t_iter.next().unwrap();
            let item_name = child.0.as_str();
            let item_val = child.1;
            match item_name {
                "match(os)" => new_arm(item_val, consts::OS),
                "match(os_family)" => new_arm(item_val, consts::FAMILY),
                _ => item_val.clone(),
            }
        }
        _ => raw_value.clone(),
    }
}
