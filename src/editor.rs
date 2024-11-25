use crossterm::event::Event;
use crossterm::event::{read, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use std::io::{self, stdout};

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Editor { should_quit: false }
    }
    pub fn initialize() -> Result<(), io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()
    }
    pub fn clear_screen() -> Result<(), io::Error> {
        execute!(stdout(), Clear(ClearType::All))
    }
    pub fn refresh_screen(&mut self) -> Result<(), io::Error> {
        if self.should_quit == true {
            Self::clear_screen()?;
            println!("Bye for now!");
        }
        Ok(())
    }
    pub fn terminate() -> Result<(), io::Error> {
        disable_raw_mode()
    }
    pub fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code,
            modifiers,
            kind,
            state,
        }) = event
        {
            println!("Code: {code:?}, Modifiers: {modifiers:?}, Kind: {kind:?}, State: {state:?}");
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => (),
            }
        }
    }
    pub fn repl(&mut self) -> Result<(), io::Error> {
        loop {
            let event = read()?;
            self.evaluate_event(&event);
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
        }

        Ok(())
    }
    pub fn run(&mut self) {
        Self::initialize().unwrap();
        let result = self.repl();
        Self::terminate().unwrap();
        result.unwrap();
    }
}
