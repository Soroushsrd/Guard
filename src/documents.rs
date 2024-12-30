use std::{char, fs, io::Write};

use crate::{
    editor::SearchDirection,
    files::FileType,
    lines::{self, Line},
    terminal::Position,
};

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
    pub fn insert(&mut self, at: &Position, c: char) {
        if at.y > self.lines.len() {
            return;
        }

        self.is_dirty = true;

        if c == '\n' {
            self.insert_new_line(at);
        } else if at.y == self.lines.len() {
            let mut line = Line::default();
            line.insert(0, c);
            self.lines.push(line);
        } else {
            let line = &mut self.lines[at.y];
            line.insert(at.x, c);
        }
        self.unhighlight_lines(at.y);
    }
    pub fn unhighlight_lines(&mut self, start: usize) {
        let start = start.saturating_sub(1);
        for line in self.lines.iter_mut().skip(start) {
            line.is_highlighted = false;
        }
    }
    pub fn save(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(file_name) = &self.file_name {
            let mut file = std::fs::File::create(file_name).unwrap();
            self.file_type = FileType::from(file_name);

            for line in &mut self.lines {
                file.write_all(line.string.as_bytes()).unwrap();
                file.write_all(b"\n").unwrap();
            }
            self.is_dirty = false;
            return Ok(());
        }
        Ok(())
    }

    pub fn highlight(&mut self, word: &Option<String>, until: &Option<usize>) {
        let mut start_comment = false;
        let until: usize = if let Some(until) = until {
            if until.saturating_add(1) < self.lines.len() {
                until.saturating_add(1)
            } else {
                self.lines.len()
            }
        } else {
            self.lines.len()
        };
        for line in &mut self.lines[..until] {
            start_comment = line.highlight(&self.file_type.highlights, word, start_comment);
        }
    }

    pub fn find(&self, query: &str, at: &Position, direction: SearchDirection) -> Option<Position> {
        if at.y >= self.lines.len() {
            return None;
        }
        let mut position = Position { x: at.x, y: at.y };
        let start = match direction {
            SearchDirection::Forward => at.y,
            SearchDirection::Backward => 0,
        };
        let end = match direction {
            SearchDirection::Forward => self.lines.len(),
            SearchDirection::Backward => at.y.saturating_add(1),
        };

        for _line in start..end {
            if let Some(line) = self.lines.get(position.y) {
                if let Some(x) = line.find(&query, position.x, direction) {
                    position.x = x;
                    return Some(position);
                }
                match direction {
                    SearchDirection::Forward => {
                        position.y = position.y.saturating_add(1);
                        position.x = 0;
                    }
                    SearchDirection::Backward => {
                        position.y = position.y.saturating_sub(1);
                        position.x = self.lines[position.y].length;
                    }
                }
            } else {
                return None;
            }
        }
        None
    }
    pub fn delete(&mut self, at: &Position) {
        let len = self.lines.len();
        if at.y >= len {
            return;
        }

        self.is_dirty = true;
        if at.x == self.lines[at.y].length && at.y + 1 < len {
            let next_line = self.lines.remove(at.y + 1);
            let line = &mut self.lines[at.y];
            line.append(&next_line);
        } else {
            let line = &mut self.lines[at.y];
            line.delete(at.x);
        }
        self.unhighlight_lines(at.y);
    }
}
