use crossterm::cursor::{Hide, Show, MoveTo};
use crossterm::queue;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size};
use crossterm::style::Print;
use std::io::{self, stdout, Write};

#[derive(Default)]
pub struct Terminal {}

#[derive(Clone, Copy, Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Terminal {
    pub fn move_cursor(position: Position) -> Result<(), io::Error> {
        let x = position.x as u16;
        let y = position.y as u16;
        queue!(stdout(), MoveTo(x, y))?;
        Ok(())
    }

    pub fn hide_cursor() -> Result<(), io::Error> {
        queue!(stdout(), Hide)?;
        Ok(())
    }

    pub fn show_cursor() -> Result<(), io::Error> {
        queue!(stdout(), Show)?;
        Ok(())
    }

    pub fn initialize() -> Result<(), io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor(Position { x: 0, y: 0 })?;
        Self::execute()
    }

    pub fn clear_screen() -> Result<(), io::Error> {
        queue!(stdout(), Clear(ClearType::All))?;
        Ok(())
    }

    pub fn terminate() -> Result<(), io::Error> {
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn print(string: &str) -> Result<(), io::Error> {
        queue!(stdout(), Print(string))?;
        Ok(())
    }

    pub fn clear_cline() -> Result<(), io::Error> {
        queue!(stdout(), Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    pub fn execute() -> Result<(), io::Error> {
        stdout().flush()?;
        Ok(())
    }

    pub fn size() -> Result<(u16, u16), io::Error> {
        let (width, height) = size()?;
        Ok((width, height))
    }
}