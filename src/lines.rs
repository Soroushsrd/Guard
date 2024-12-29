use crossterm::style::Color;
use std::{cmp, str::FromStr};
use unicode_segmentation::UnicodeSegmentation;

use crate::highlights::Type;

#[derive(Default)]
pub struct Line {
    pub string: String,
    pub highlighting: Vec<Type>,
    pub is_highlighted: bool,
    length: usize,
}

impl From<&str> for Line {
    fn from(value: &str) -> Self {
        Self {
            string: String::from(value),
            highlighting: Vec::new(),
            is_highlighted: false,
            length: value.graphemes(true).count(),
        }
    }
}

impl Line {
    pub fn split(&mut self, at: usize) -> Self {
        let mut line = String::new();

        let mut length = 0;
        let mut splitted_line = String::new();
        let mut splitted_length = 0;

        for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
            if index < at {
                length += 1;
                line.push_str(grapheme);
            } else {
                splitted_length += 1;
                splitted_line.push_str(grapheme);
            }
        }
        self.string = line;
        self.length = length;
        self.is_highlighted = false;
        Self {
            string: splitted_line,
            length: splitted_length,
            is_highlighted: false,
            highlighting: Vec::new(),
        }
    }
}
