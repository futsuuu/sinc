use std::io::{stdout, Error, Write};

use crossterm::{
    cursor, execute, queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal,
};

pub struct Progress {
    len: u16,
    pub val: u16,
    pub message: String,
}

impl Progress {
    pub fn new(len: u16) -> Self {
        queue!(stdout(), Print("\n\n\n"), cursor::Hide).unwrap();
        Self {
            len,
            val: 0,
            message: "".to_string(),
        }
    }

    pub fn draw(&mut self) -> Result<(), Error> {
        self.message.truncate(self.max_len().into());
        let len = (self.get_per() * self.max_len()) / 100;
        queue!(
            stdout(),
            cursor::MoveUp(2),
            cursor::MoveToColumn(2),
            Print(" ".repeat(self.max_len().into())),
            cursor::MoveToColumn(2),
            Print(self.message.to_string()),
            cursor::MoveToNextLine(1),
            cursor::MoveToColumn(2),
            SetForegroundColor(Color::DarkGrey),
            Print(self.bar("━", "━", "━", self.max_len())),
            cursor::MoveToColumn(2),
            SetForegroundColor(Color::Blue),
            Print(self.bar("━", "━", "━", len)),
            ResetColor,
            cursor::MoveToNextLine(1),
            cursor::MoveToColumn(2),
            Print(format!("{}%", self.get_per())),
        )?;
        stdout().flush()?;
        Ok(())
    }

    pub fn end(&mut self) -> Result<(), Error> {
        self.draw()?;
        execute!(stdout(), cursor::Show)?;
        Ok(())
    }

    fn bar(&self, s1: &str, s2: &str, s3: &str, len: u16) -> String {
        let length = if len > 2 { len - 2 } else { len };
        format!("{}{}{}", s1, s2.repeat(length.into()), s3)
    }

    fn get_per(&self) -> u16 {
        self.val * 100 / self.len
    }

    fn max_len(&self) -> u16 {
        terminal::size().unwrap().0 - 4
    }
}
