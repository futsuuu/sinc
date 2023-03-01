use std::{fmt::Display, path::PathBuf};

use crossterm::{
    style::{style, Stylize},
    terminal,
};

pub fn symbol(symbol: &str) {
    print!("{}", symbol.cyan())
}

pub fn item_type<D>(item_type: D)
where
    D: Display,
{
    print!("{}", style(item_type).magenta().bold())
}

pub fn path(path: &PathBuf) {
    let backquote = "`".green().dim();
    print!(
        "{}{}{}",
        backquote,
        path.display().to_string().green(),
        backquote
    )
}

pub fn title<D>(title: D)
where
    D: Display,
{
    print!(
        "{}{}",
        format!("▂\n█  {}\n▔", title).cyan().bold(),
        "▔"
            .repeat((terminal::size().unwrap().0 - 1).into())
            .dark_grey()
    );
}
