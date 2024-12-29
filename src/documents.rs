use std::{char, fs};

use crate::{files::FileType, lines::Line, terminal::Position};

#[derive(Default)]
pub struct Document {
    pub lines: Vec<Line>,
    pub file_name: Option<String>,
    pub is_dirty: bool,
    pub file_type: FileType,
}

impl Document {
    pub fn open(file_name: &str) -> Result<Self, std::io::Error> {
        let contents = fs::read_to_string(file_name)?;
        let file_type = FileType::from(file_name);

        let mut lines = Vec::new();
        for line in contents.lines() {
            lines.push(Line::from(line));
        }
        Ok(Self {
            lines,
            file_name: Some(file_name.to_string()),
            is_dirty: false,
            file_type,
        })
    }
    pub fn insert_new_line(&mut self, at: &Position) -> Result<(), Box<dyn std::error::Error>> {
        if at.y > self.lines.len() {
            return Err("Position is larget than the line length!".into());
        }

        if at.y == self.lines.len() {
            self.lines.push(Line::default());
            return Ok(());
        }

        let current_line = &mut self.lines[at.y];
        let new_line = current_line.split(at.x);
        self.lines.insert(at.y + 1, new_line);
        Ok(())
    }
}
