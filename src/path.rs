use std::{env, fs, path::PathBuf};

use anyhow::Result;
use dirs::home_dir;

pub fn config_file() -> String {
    let config_dir = match env::var("SINC_CONFIG_DIR") {
        Ok(p) => p,
        Err(_) => match env::var("XDG_CONFIG_HOME") {
            Ok(p) => p,
            Err(_) => "~/.config".to_string(),
        },
    };
    to_correct(format!("{}/sinc/sinc.toml", config_dir))
}

pub fn cache_file(file_name: &str) -> Result<PathBuf> {
    let cache_dir = match env::var("XDG_CACHE_HOME") {
        Ok(p) => p,
        Err(_) => "~/.cache".to_string(),
    };
    let r = PathBuf::from(to_correct(cache_dir + "/sinc/" + file_name));
    let parent_dir = r.parent().unwrap();
    if !parent_dir.exists() {
        fs::create_dir_all(parent_dir)?;
    }
    Ok(r)
}

pub fn to_correct(path: String) -> String {
    let sep = if cfg!(target_os = "windows") {
        ("/", "\\")
    } else {
        ("\\", "/")
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

pub fn omit_home(path: String) -> String {
    path.replacen(home_dir().unwrap().to_str().unwrap(), "~", 1)
}
