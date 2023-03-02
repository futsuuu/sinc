use std::{fmt::Display, path::Path};

use crossterm::{
    style::{style, StyledContent, Stylize},
    terminal,
};

use crate::path::omit_home;

pub fn symbol(symbol: &str) -> StyledContent<&str> {
    symbol.cyan()
}

pub fn item_type<D>(item_type: D) -> StyledContent<D>
where
    D: Display,
{
    style(item_type).magenta().bold()
}

pub fn path(path: &Path) -> String {
    let backquote = "`".green().dim();
    format!(
        "{}{}{}",
        backquote,
        omit_home(path.display().to_string()).green(),
        backquote
    )
}

pub fn title<D>(title: D) -> String
where
    D: Display,
{
    format!(
        "{}{}",
        format!("▂\n█  {}\n▔", title).cyan().bold(),
        "▔"
            .repeat((terminal::size().unwrap().0 - 1).into())
            .dark_grey()
    )
}
