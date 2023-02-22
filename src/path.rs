use std::env;

use dirs::home_dir;

pub fn config_file() -> String {
    format!("{}/singer.toml", get_dir("XDG_CONFIG_HOME", "~/.config"))
}

fn get_dir(env_var: &str, fallback: &str) -> String {
    to_correct(format!(
        "{}/singer",
        match env::var(env_var) {
            Ok(p) => p,
            Err(_) => fallback.to_string(),
        }
    ))
}

pub fn to_correct(path: String) -> String {
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
