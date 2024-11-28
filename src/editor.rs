use crate::terminal::{Position, Terminal};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, read};
use std::io::{self, Write};

pub struct Editor {
    should_quit: bool,
    content: String,
    position: Position,
}

impl Editor {
    pub fn new() -> Self {
        Editor {
            should_quit: false,
            content: String::new(),
            position: Position::default(),
        }
    }

    pub fn refresh_screen(&mut self) -> Result<(), io::Error> {
        Terminal::hide_cursor()?;

        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Bye for now!")?;
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor(Position { x: 0, y: 0 })?;
            Terminal::print(&self.content)?;
            Terminal::move_cursor(self.position)?;
        }

        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }

    pub fn evaluate_event(&mut self, event: &Event) {
        if let Event::Key(KeyEvent { code, modifiers, kind, state }) = event {
            match code {
                KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                KeyCode::Char(c) if *kind == KeyEventKind::Release => {
                    self.content.push(*c);
                    self.position.x += 1;
                }
                KeyCode::Backspace if self.position.x > 0 => {
                    self.content.pop();
                    self.position.x -= 1;
                }
                KeyCode::Enter => {
                    self.content.push('\n');
                    self.position.y += 1;
                    self.position.x = 0;
                }
                KeyCode::Left if self.position.x > 0 => {
                    self.position.x -= 1;
                }
                KeyCode::Right => {
                    self.position.x += 1;
                }
                KeyCode::Up if self.position.y > 0 => {
                    self.position.y -= 1;
                }
                KeyCode::Down => {
                    self.position.y += 1;
                }
                _ => (),
            }
        }
    }

    pub fn repl(&mut self) -> Result<(), io::Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }

            let event = read()?;
            self.evaluate_event(&event);
        }

        Ok(())
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    pub fn welcome_msg() -> Result<(), io::Error> {
        let (t_x, _) = Terminal::size()?;
        let width = "Stop Talking, Code Instead!".len();
        let padding = (t_x - width as u16) / 2;
        let spaces = " ".repeat(padding as usize);
        let welcome_msg = &format!("{spaces}Stop Talking, Code Instead!");
        Terminal::print(welcome_msg)?;
        Ok(())
    }

    pub fn draw_rows() -> Result<(), io::Error> {
        let terminal_size = Terminal::size()?;
        for current_row in 0..terminal_size.1 {
            Terminal::clear_cline()?;
            if current_row == terminal_size.1 / 3 {
                Self::welcome_msg()?;
            } else {
                Terminal::print("~")?;
            }
            if current_row + 1 < terminal_size.1 {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }
}