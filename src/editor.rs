use crate::terminal::{Position, Terminal};
use crossterm::event::Event;
use crossterm::event::{read, Event::Key, KeyCode::Char, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::terminal::size;
use std::io::{self};

///////////////////////////////////////////
///need to handle execute. either execute or printing the string should be removed
//////////////////////////////////////////
/// have to handle the tilde printing. right now it skips one line when enter is pressed
////////////////////////////////////////
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
            position: Position::default(),
        }
    }
    pub fn refresh_screen(&mut self) -> Result<(), io::Error> {
        Terminal::hide_cursor()?;
        if self.should_quit == true {
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
        //let x_pos = t_x / 2;
        //let y_pos = t_y / 3;
        let width = "Stop Talking, Code Instead!".len();
        let padding = (t_x - width as u16) / 2;
        let spaces = " ".repeat(padding as usize - 1);
        let welcome_msg = &format!("{spaces}Stop Talking, Code Instead!");
        Terminal::print(welcome_msg)?;
        Ok(())
    }
    pub fn draw_rows() -> Result<(), io::Error> {
        let terminal_szie = size()?;
        for current_row in 0..terminal_szie.1 {
            Terminal::clear_cline()?;
            if current_row == terminal_szie.1 / 3 {
                Self::welcome_msg()?;
            } else {
                Terminal::print("~")?;
            }
            if current_row + 1 < terminal_szie.1 {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }
}
