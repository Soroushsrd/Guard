use crossterm::style::Color;
use std::char;
use unicode_segmentation::UnicodeSegmentation;

use crate::{
    editor::SearchDirection, files::HighLightsOptions, highlights::Type, terminal::Position,
};

#[derive(Default)]
pub struct Line {
    pub string: String,
    pub highlighting: Vec<Type>,
    pub is_highlighted: bool,
    pub length: usize,
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
    pub fn highlight(
        &mut self,
        options: &HighLightsOptions,
        word: &Option<String>,
        start_with_comment: bool,
    ) -> bool {
        let chars: Vec<char> = self.string.chars().collect();

        if self.is_highlighted && word.is_none() {
            if let Some(hl_type) = self.highlighting.last() {
                if *hl_type == Type::MultilineComment
                    && self.string.len() > 1
                    && self.string[self.string.len() - 2..] == *"*/"
                {
                    return true;
                }
            }
            return false;
        }

        self.highlighting = Vec::new();

        let mut index = 0;
        let mut inside_ml_comment = start_with_comment;

        if inside_ml_comment {
            let closing_index = if let Some(closing_index) = self.string.find("*/") {
                closing_index + 2
            } else {
                chars.len()
            };

            for _ in 0..closing_index {
                self.highlighting.push(Type::MultilineComment);
            }
            index = closing_index;
        }

        while let Some(c) = chars.get(index) {
            if self.highlight_multiline_comments(&mut index, options, *c, &chars) {
                inside_ml_comment = true;
                continue;
            }
            inside_ml_comment = false;

            if self.highlight_character(&mut index, options, *c, &chars)
                || self.highlight_comment(&mut index, options, *c, &chars)
                || self.highlight_primary_keyword(&mut index, options, &chars)
                || self.highlight_secondary_keywords(&mut index, options, &chars)
                || self.highlight_strings(&mut index, options, *c, &chars)
                || self.highlight_number(&mut index, options, *c, &chars)
            {
                continue;
            }
            self.highlighting.push(Type::None);
            index += 1;
        }
        self.highlight_match(word);
        if inside_ml_comment && &self.string[self.string.len().saturating_sub(2)..] != "*/" {
            return true;
        }
        self.is_highlighted = true;
        false
    }
    pub fn highlight_match(&mut self, word: &Option<String>) {
        if let Some(word) = word {
            if word.is_empty() {
                return;
            }
            let mut index = 0;
            while let Some(search_match) = self.find(word, index, SearchDirection::Forward) {
                if let Some(nex_index) = search_match.checked_add(word[..].graphemes(true).count())
                {
                    for i in search_match..nex_index {
                        self.highlighting[i] = Type::Match;
                    }
                    index = nex_index;
                } else {
                    break;
                }
            }
        }
    }
    pub fn highlight_str(
        &mut self,
        index: &mut usize,
        substring: &str,
        chars: &[char],
        highlight_type: Type,
    ) -> bool {
        if substring.is_empty() {
            return false;
        }

        for (substring_index, c) in substring.chars().enumerate() {
            if let Some(next_char) = chars.get(index.saturating_add(substring_index)) {
                if *next_char != c {
                    return false;
                }
            } else {
                return false;
            }
        }
        for _ in 0..substring.len() {
            self.highlighting.push(highlight_type);
            *index += 1;
        }
        true
    }
    pub fn highlight_keywords(
        &mut self,
        index: &mut usize,
        chars: &[char],
        keywords: &[String],
        hl_type: Type,
    ) -> bool {
        if *index > 0 {
            let prev = chars[*index - 1];
            if !prev.is_ascii_punctuation() || !prev.is_ascii_whitespace() {
                return false;
            }
        }
        for word in keywords {
            if *index < chars.len().saturating_sub(word.len()) {
                let next_char = chars[*index + word.len()];
                if !next_char.is_ascii_whitespace() || !next_char.is_ascii_punctuation() {
                    continue;
                }
            }

            if self.highlight_str(index, &word, chars, hl_type) {
                return true;
            }
        }
        false
    }

    pub fn highlight_primary_keyword(
        &mut self,
        index: &mut usize,
        options: &HighLightsOptions,
        chars: &[char],
    ) -> bool {
        self.highlight_keywords(
            index,
            chars,
            &options.primrary_keywords,
            Type::PrimaryKeywords,
        )
    }

    pub fn highlight_secondary_keywords(
        &mut self,
        index: &mut usize,
        opts: &HighLightsOptions,
        chars: &[char],
    ) -> bool {
        self.highlight_keywords(
            index,
            chars,
            &opts.secondary_keywords,
            Type::SecondaryKeywords,
        )
    }

    pub fn highlight_character(
        &mut self,
        index: &mut usize,
        options: &HighLightsOptions,
        c: char,
        chars: &[char],
    ) -> bool {
        if options.character && c == '\'' {
            if let Some(next_char) = chars.get(index.saturating_add(1)) {
                let end_char_index = if *next_char == '\\' {
                    index.saturating_add(3)
                } else {
                    index.saturating_add(2)
                };
                if let Some(end_char) = chars.get(end_char_index) {
                    if *end_char == '\'' {
                        for _ in 0..=end_char_index.saturating_sub(*index) {
                            self.highlighting.push(Type::Character);
                            *index += 1;
                        }
                        return true;
                    }
                }
            }
        }
        false
    }
    pub fn highlight_comment(
        &mut self,
        index: &mut usize,
        opts: &HighLightsOptions,
        c: char,
        chars: &[char],
    ) -> bool {
        if opts.comments && c == '/' && *index < chars.len() {
            if let Some(next_char) = chars.get(index.saturating_add(1)) {
                if *next_char == '/' {
                    for _ in *index..chars.len() {
                        self.highlighting.push(Type::Comment);
                        *index += 1;
                    }
                    return true;
                }
            };
        }
        false
    }

    pub fn highlight_multiline_comments(
        &mut self,
        index: &mut usize,
        options: &HighLightsOptions,
        c: char,
        chars: &[char],
    ) -> bool {
        if options.comments && c == '/' && *index < chars.len() {
            if let Some(next_char) = chars.get(index.saturating_add(1)) {
                if *next_char == '*' {
                    let closing_index =
                        if let Some(closing_index) = self.string[*index + 2..].find("*/") {
                            *index + closing_index + 4
                        } else {
                            chars.len()
                        };
                    for _ in *index..closing_index {
                        self.highlighting.push(Type::MultilineComment);
                        *index += 1;
                    }
                    return true;
                }
            }
        }
        false
    }

    pub fn highlight_strings(
        &mut self,
        index: &mut usize,
        options: &HighLightsOptions,
        c: char,
        chars: &[char],
    ) -> bool {
        if options.strings && c == '"' {
            loop {
                self.highlighting.push(Type::String);
                *index += 1;
                if let Some(next_char) = chars.get(*index) {
                    if *next_char == '"' {
                        break;
                    }
                } else {
                    break;
                }
            }
            self.highlighting.push(Type::String);
            *index += 1;
            return true;
        }
        false
    }
    pub fn highlight_number(
        &mut self,
        index: &mut usize,
        opts: &HighLightsOptions,
        c: char,
        chars: &[char],
    ) -> bool {
        if opts.numbers && c.is_ascii_digit() {
            if *index > 0 {
                let prev_char = chars[*index - 1];
                if !prev_char.is_ascii_punctuation() || !prev_char.is_ascii_whitespace() {
                    return false;
                }
            }

            loop {
                self.highlighting.push(Type::Number);
                *index += 1;

                if let Some(next_char) = chars.get(*index) {
                    if *next_char != '.' && !next_char.is_ascii_digit() {
                        break;
                    }
                } else {
                    break;
                }
            }
            return true;
        }
        false
    }
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
    pub fn append(&mut self, new_line: &Self) {
        self.string = format!("{}{}", self.string, new_line.string);
        self.length += new_line.length;
    }

    pub fn delete(&mut self, at: usize) {
        if at >= self.length {
            return;
        }
        let mut end_string = String::new();
        let mut end_length = 0;
        for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
            if index != at {
                end_length += 1;
                end_string.push_str(grapheme);
            }
        }
        self.length = end_length;
        self.string = end_string;
    }

    pub fn find(&self, query: &str, at: usize, direction: SearchDirection) -> Option<usize> {
        if at > self.length || query.is_empty() {
            return None;
        }
        let start = match direction {
            SearchDirection::Forward => at,
            SearchDirection::Backward => 0,
        };
        let end = match direction {
            SearchDirection::Forward => self.length,
            SearchDirection::Backward => at,
        };
        let substring: String = self.string[..]
            .graphemes(true)
            .skip(start)
            .take(end - start)
            .collect();

        match direction {
            SearchDirection::Forward => {
                if let Some(match_idx) = substring.find(query) {
                    let grapheme_count = substring[..match_idx].graphemes(true).count();
                    Some(start + grapheme_count)
                } else {
                    None
                }
            }
            SearchDirection::Backward => {
                if let Some(match_idx) = substring.rfind(query) {
                    let grapheme_count = substring[..match_idx].graphemes(true).count();
                    Some(start + grapheme_count)
                } else {
                    None
                }
            }
        }
    }
    pub fn insert(&mut self, at: usize, c: char) {
        if at >= self.length {
            self.string.push(c);
            self.length += 1;
            return;
        }

        let mut result = String::new();
        let mut length = 0;

        for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
            length += 1;
            if index == at {
                length += 1;
                result.push(c);
            }
            result.push_str(grapheme);
        }
        self.length = length;
        self.string = result;
    }
}
