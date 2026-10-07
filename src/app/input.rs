//! Single-line input editor with history.

use unicode_width::UnicodeWidthStr;

#[derive(Default)]
pub struct Input {
    text: String,
    /// Byte offset of the cursor; always on a char boundary.
    cursor: usize,
    history: Vec<String>,
    /// Position while walking history; `None` when editing a fresh line.
    hist_pos: Option<usize>,
    draft: String,
}

impl Input {
    pub fn with_history(history: Vec<String>) -> Input {
        Input {
            history,
            ..Default::default()
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    /// Display columns before the cursor.
    pub fn cursor_col(&self) -> usize {
        self.text[..self.cursor].width()
    }

    pub fn set(&mut self, text: impl Into<String>) {
        self.text = text.into();
        self.cursor = self.text.len();
    }

    pub fn take(&mut self) -> String {
        let line = std::mem::take(&mut self.text);
        self.cursor = 0;
        self.hist_pos = None;
        if !line.trim().is_empty() && self.history.last() != Some(&line) {
            self.history.retain(|h| h != &line);
            self.history.push(line.clone());
        }
        line
    }

    pub fn insert(&mut self, s: &str) {
        self.text.insert_str(self.cursor, s);
        self.cursor += s.len();
    }

    pub fn backspace(&mut self) {
        if let Some(prev) = self.prev_boundary() {
            self.text.replace_range(prev..self.cursor, "");
            self.cursor = prev;
        }
    }

    pub fn delete(&mut self) {
        if let Some(next) = self.next_boundary() {
            self.text.replace_range(self.cursor..next, "");
        }
    }

    pub fn left(&mut self) {
        if let Some(p) = self.prev_boundary() {
            self.cursor = p;
        }
    }

    pub fn right(&mut self) {
        if let Some(n) = self.next_boundary() {
            self.cursor = n;
        }
    }

    pub fn home(&mut self) {
        self.cursor = 0;
    }

    pub fn end(&mut self) {
        self.cursor = self.text.len();
    }

    pub fn kill_to_start(&mut self) {
        self.text.replace_range(..self.cursor, "");
        self.cursor = 0;
    }

    pub fn kill_to_end(&mut self) {
        self.text.truncate(self.cursor);
    }

    pub fn delete_word(&mut self) {
        let before = &self.text[..self.cursor];
        let trimmed = before.trim_end();
        let start = trimmed
            .rfind(|c: char| c.is_whitespace())
            .map(|i| i + 1)
            .unwrap_or(0);
        self.text.replace_range(start..self.cursor, "");
        self.cursor = start;
    }

    pub fn history_prev(&mut self) {
        if self.history.is_empty() {
            return;
        }
        let pos = match self.hist_pos {
            None => {
                self.draft = self.text.clone();
                self.history.len() - 1
            }
            Some(0) => 0,
            Some(p) => p - 1,
        };
        self.hist_pos = Some(pos);
        self.set(self.history[pos].clone());
    }

    pub fn history_next(&mut self) {
        let Some(pos) = self.hist_pos else { return };
        if pos + 1 < self.history.len() {
            self.hist_pos = Some(pos + 1);
            self.set(self.history[pos + 1].clone());
        } else {
            self.hist_pos = None;
            let draft = std::mem::take(&mut self.draft);
            self.set(draft);
        }
    }

    fn prev_boundary(&self) -> Option<usize> {
        self.text[..self.cursor]
            .char_indices()
            .last()
            .map(|(i, _)| i)
    }

    fn next_boundary(&self) -> Option<usize> {
        self.text[self.cursor..]
            .chars()
            .next()
            .map(|c| self.cursor + c.len_utf8())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edits_and_walks_history() {
        let mut i = Input::with_history(vec!["/list".into(), "/show 1".into()]);
        i.insert("/he");
        i.history_prev();
        assert_eq!(i.text(), "/show 1");
        i.history_prev();
        assert_eq!(i.text(), "/list");
        i.history_next();
        i.history_next();
        assert_eq!(i.text(), "/he");
        i.insert("lp me");
        i.delete_word();
        assert_eq!(i.text(), "/help ");
        i.backspace();
        i.left();
        i.insert("é");
        assert_eq!(i.text(), "/helép");
        assert_eq!(i.take(), "/helép");
        assert_eq!(i.text(), "");
    }
}
