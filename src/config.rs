use std::{env::consts, fs};

use anyhow::{Context, Result};
use pathsearch::find_executable_in_path;
use serde::Deserialize;
use toml::{value, Value};

use crate::path;

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
    pub enable: bool,
    pub hook_add: String,
}

pub fn load_config(config_path: String) -> Result<Config> {
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
        let s = fs::read_to_string(&config_path)
            .with_context(|| format!("failed to read {}", config_path))?;
        toml::from_str(&s).context("failed to deserialize sinc.toml")?
    };

    let mut dotfiles = Vec::new();
    for df in user_config.dotfiles {
        let dir = get_val(&df, "dir", Some(&user_config.default.dir));
        let sync_type = get_val(&df, "sync_type", Some(&user_config.default.sync_type));
        let path = get_val(&df, "path", None);
        let target = get_val(&df, "target", None);
        let enable = get_val(&df, "enable", Some(&Value::Boolean(true)));
        let hook_add = get_val(&df, "hook_add", Some(&Value::from("")));
        dotfiles.push(Dotfile {
            dir: val2string(dir),
            path: val2string(path),
            target: val2string(target),
            sync_type: val2string(sync_type),
            enable: enable.as_bool().unwrap(),
            hook_add: val2string(hook_add),
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
            let (item_name, item_val) = t_iter.next().unwrap();
            let (func_name, func_val) = item_name // "sys(os)"
                .rsplit_once(')')
                .unwrap_or_default() //             ("sys(os", "")
                .0 //                                "sys(os"
                .split_once('(')
                .unwrap_or_default(); //            ("sys", "os")
            match func_name {
                "sys" => match func_val {
                    "os_type" => new_arm(item_val, consts::OS),
                    "os_family" => new_arm(item_val, consts::FAMILY),
                    "os" => new_arm(
                        item_val,
                        format!("{}", os_info::get().os_type())
                            .to_lowercase()
                            .as_str(),
                    ),
                    _ => item_val.clone(),
                },
                "which" => {
                    let mut correct_path_val = value::Table::new();
                    for (key, val) in item_val.as_table().unwrap() {
                        correct_path_val.insert(path::to_correct(key.clone()), val.clone());
                    }
                    new_arm(
                        &Value::Table(correct_path_val),
                        find_executable_in_path(func_val)
                            .unwrap_or_default()
                            .to_str()
                            .unwrap_or_default(),
                    )
                }
                _ => item_val.clone(),
            }
        }
        _ => raw_value.clone(),
    }
}
