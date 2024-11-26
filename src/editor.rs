use crossterm::event::Event;
use crossterm::event::{read, Event::Key, KeyCode::Char, KeyEvent, KeyEventKind, KeyModifiers};
use std::io::{self};

use crate::terminal::{Position, Terminal};

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
            Terminal::draw_rows()?;
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
}
