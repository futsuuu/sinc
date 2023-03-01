use std::fmt::Display;

use crossterm::{style::Stylize, terminal};

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
