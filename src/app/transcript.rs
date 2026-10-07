//! The scrollable transcript: structured entries laid out lazily at the
//! current width, so resizing reflows everything.

use ratatui::text::Line;

use crate::ui::text::{Para, layout};

pub struct Entry {
    pub paras: Vec<Para>,
    /// Plain-text version for `/copy`. Empty for entries not worth copying.
    pub copy: String,
    /// Echo of user input; drawn without a blank line after it.
    pub input: bool,
}

#[derive(Default)]
pub struct Transcript {
    entries: Vec<Entry>,
    cache: Option<(usize, Vec<Line<'static>>)>,
    /// First line of each entry in the cached layout.
    starts: Vec<usize>,
    /// Entry to bring to the top of the viewport on the next draw, so long
    /// output is read from its beginning.
    anchor: Option<usize>,
    /// Lines scrolled up from the bottom. 0 follows new output.
    scroll: usize,
}

impl Transcript {
    pub fn push(&mut self, entry: Entry) {
        if entry.input {
            self.anchor = Some(self.entries.len());
        }
        self.entries.push(entry);
        self.cache = None;
        self.scroll = 0;
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        self.cache = None;
        self.anchor = None;
        self.scroll = 0;
    }

    /// Text of the most recent entry that has any.
    pub fn last_copy(&self) -> Option<&str> {
        self.entries
            .iter()
            .rev()
            .map(|e| e.copy.as_str())
            .find(|c| !c.is_empty())
    }

    pub fn lines(&mut self, width: usize) -> &[Line<'static>] {
        if self.cache.as_ref().is_none_or(|(w, _)| *w != width) {
            let mut lines = Vec::new();
            self.starts.clear();
            for (i, e) in self.entries.iter().enumerate() {
                self.starts.push(lines.len());
                lines.extend(layout(&e.paras, width));
                let next_is_output = self.entries.get(i + 1).is_some_and(|n| !n.input);
                if !(e.input && next_is_output) {
                    lines.push(Line::default());
                }
            }
            self.cache = Some((width, lines));
        }
        &self.cache.as_ref().unwrap().1
    }

    pub fn scroll(&self) -> usize {
        self.scroll
    }

    pub fn scroll_by(&mut self, delta: isize, max: usize) {
        self.scroll = self.scroll.saturating_add_signed(delta).min(max);
    }

    pub fn scroll_to_bottom(&mut self) {
        self.scroll = 0;
    }

    /// Clamps scrolling to the content and applies a pending anchor.
    /// Call after `lines` for the current width.
    pub fn settle(&mut self, height: usize) {
        let total = self.cache.as_ref().map_or(0, |(_, l)| l.len());
        if let Some(i) = self.anchor.take()
            && let Some(&start) = self.starts.get(i)
        {
            self.scroll = total.saturating_sub(start + height);
        }
        self.scroll = self.scroll.min(total.saturating_sub(height));
    }
}
