use std::env;

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
