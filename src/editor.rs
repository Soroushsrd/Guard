use crate::terminal::{Position, Terminal};
use crossterm::event::Event;
use crossterm::event::{read, Event::Key, KeyCode::Char, KeyEvent, KeyEventKind, KeyModifiers};
use std::fmt::format;
use std::io::{self};
use std::usize;

///////////////////////////////////////////
///need to handle execute. either execute or printing the string should be removed
//////////////////////////////////////////
/// have to handle the tilde printing. right now it skips one line when enter is pressed
////////////////////////////////////////
const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
    content: String,
    position: Position,
}

impl Editor {
    pub fn default() -> Self {
        Editor {
            should_quit: false,
            content: String::new(),
            position: Position { x: 0, y: 0 },
        }
    }
    pub fn refresh_screen(&mut self) -> Result<(), io::Error> {
        Terminal::hide_cursor()?;

        if self.should_quit == true {
            Terminal::clear_screen()?;

            Terminal::print("Bye for now!")?;
        } else {
            self.draw_rows()?;
            Terminal::move_cursor(Position { x: 0, y: 0 })?;
            Terminal::print(&self.content)?;
            Terminal::move_cursor(self.position)?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }
    pub fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code,
            modifiers,
            kind,
            state,
        }) = event
        {
            //println!(
            //    "Code: {code:#?}, Modifier: {modifiers:#?}, Kind: {kind:#?}, State: {state:#?}"
            //);
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                Char(c) if *kind == KeyEventKind::Release => {
                    self.content.push(*c);
                    self.position.x += 1;
                }
                crossterm::event::KeyCode::Enter => {
                    self.content.push('\n');
                    self.position.y += 1;
                    self.position.x = 0;
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
        let (t_x, _t_y) = size()?;
        let version_msg = &format!("{NAME} Editor -- version {VERSION}");
        let v_msg_width = version_msg.len();
        let v_msg_padding = (t_x - v_msg_width as u16) / 2;
        let v_spaces = " ".repeat(v_msg_padding as usize - 1);
        let v_msg_string = &format!("~{v_spaces}{version_msg}\n");

        let width = "Stop Talking, Code Instead!".len();
        let padding = (t_x - width as u16) / 2;
        let spaces = " ".repeat(padding as usize - 1);
        let welcome_msg = &format!("~{spaces}Stop Talking, Start Coding!");
        Terminal::print(v_msg_string)?;
        Terminal::print(welcome_msg)?;
        Ok(())
    }
    pub fn draw_rows(&self) -> Result<(), io::Error> {
        let terminal_szie = size()?;
        let content_lines = self.content.matches('\n').count() + 1;

        for current_row in 0..terminal_szie.1 {
            Terminal::clear_cline()?;

            if current_row == terminal_szie.1 / 3 && content_lines <= 1 {
                Self::welcome_msg()?;
            } else if current_row as usize >= content_lines {
                Terminal::print("~")?;
            }
            if current_row + 1 < terminal_szie.1 {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }
}
