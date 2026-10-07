//! Width-aware text layout. Transcript entries produce `Para`s; `layout`
//! wraps them to the current width with hanging indents. Wrapping ourselves
//! (instead of letting ratatui do it) gives exact line counts for scrolling.

use ratatui::style::Style;
use ratatui::text::{Line, Span};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

#[derive(Debug, Clone, Default)]
pub struct Para {
    /// Drawn before the first line.
    pub prefix: Vec<Span<'static>>,
    /// Drawn before every wrapped continuation line.
    pub cont: Vec<Span<'static>>,
    pub spans: Vec<Span<'static>>,
    /// `false` hard-breaks at the width instead of at word boundaries (code).
    pub words: bool,
}

impl Para {
    pub fn new(spans: Vec<Span<'static>>) -> Para {
        Para {
            spans,
            words: true,
            ..Default::default()
        }
    }

    pub fn plain(text: impl Into<String>, style: Style) -> Para {
        Para::new(vec![Span::styled(text.into(), style)])
    }

    pub fn blank() -> Para {
        Para::new(vec![])
    }

    /// Indent both the first and continuation lines.
    pub fn indent(mut self, n: usize) -> Para {
        let pad = " ".repeat(n);
        self.prefix.insert(0, Span::raw(pad.clone()));
        self.cont.insert(0, Span::raw(pad));
        self
    }

    pub fn with_prefix(mut self, prefix: Vec<Span<'static>>, cont: Vec<Span<'static>>) -> Para {
        self.prefix.extend(prefix);
        self.cont.extend(cont);
        self
    }

    pub fn code(mut self) -> Para {
        self.words = false;
        self
    }

    pub fn plain_text(&self) -> String {
        self.spans.iter().map(|s| s.content.as_ref()).collect()
    }
}

fn spans_width(spans: &[Span]) -> usize {
    spans.iter().map(|s| s.content.width()).sum()
}

/// Wraps paragraphs to `width` columns.
pub fn layout(paras: &[Para], width: usize) -> Vec<Line<'static>> {
    let mut out = Vec::new();
    for p in paras {
        wrap(p, width.max(8), &mut out);
    }
    out
}

fn wrap(p: &Para, width: usize, out: &mut Vec<Line<'static>>) {
    let first_w = spans_width(&p.prefix);
    let cont_w = spans_width(&p.cont);
    let avail_for = |first: bool| {
        width
            .saturating_sub(if first { first_w } else { cont_w })
            .max(1)
    };

    // Tokens are words or runs of spaces (or whole spans for code).
    let mut tokens: Vec<(String, Style)> = Vec::new();
    for span in &p.spans {
        let mut cur = String::new();
        let mut cur_space = None;
        for ch in span.content.chars() {
            let is_space = ch == ' ';
            if p.words && cur_space.is_some_and(|s| s != is_space) {
                tokens.push((std::mem::take(&mut cur), span.style));
            }
            cur.push(ch);
            cur_space = Some(is_space);
        }
        if !cur.is_empty() {
            tokens.push((cur, span.style));
        }
    }

    let mut lines: Vec<Vec<Span<'static>>> = Vec::new();
    let mut cur: Vec<Span<'static>> = Vec::new();
    let mut cur_w = 0;
    let mut avail = avail_for(true);

    for (text, style) in tokens {
        let w = text.width();
        if p.words {
            if cur_w + w <= avail {
                cur.push(Span::styled(text, style));
                cur_w += w;
                continue;
            }
            if text.starts_with(' ') {
                // Wrap point: the space is dropped.
                lines.push(std::mem::take(&mut cur));
                cur_w = 0;
                avail = avail_for(false);
                continue;
            }
            if cur_w > 0 && w <= avail_for(false) {
                lines.push(std::mem::take(&mut cur));
                avail = avail_for(false);
                cur.push(Span::styled(text, style));
                cur_w = w;
                continue;
            }
        }
        // Hard-break by characters: code, or a word wider than a line.
        let mut chunk = String::new();
        for ch in text.chars() {
            let cw = ch.width().unwrap_or(0);
            if cur_w + cw > avail && cur_w > 0 {
                if !chunk.is_empty() {
                    cur.push(Span::styled(std::mem::take(&mut chunk), style));
                }
                lines.push(std::mem::take(&mut cur));
                cur_w = 0;
                avail = avail_for(false);
            }
            chunk.push(ch);
            cur_w += cw;
        }
        if !chunk.is_empty() {
            cur.push(Span::styled(chunk, style));
        }
    }
    lines.push(cur);

    for (i, mut spans) in lines.into_iter().enumerate() {
        if p.words {
            while spans
                .last()
                .is_some_and(|s| s.content.trim_end().is_empty())
            {
                spans.pop();
            }
            if let Some(last) = spans.last_mut() {
                let trimmed = last.content.trim_end().to_string();
                last.content = trimmed.into();
            }
        }
        let mut line = if i == 0 {
            p.prefix.clone()
        } else {
            p.cont.clone()
        };
        line.extend(spans);
        out.push(Line::from(line));
    }
}

/// Pads or truncates `s` to exactly `width` columns, adding `…` when cut.
pub fn fit(s: &str, width: usize) -> String {
    let w = s.width();
    if w <= width {
        return format!("{s}{}", " ".repeat(width - w));
    }
    let mut out = String::new();
    let mut used = 0;
    for ch in s.chars() {
        let cw = ch.width().unwrap_or(0);
        if used + cw + 1 > width {
            break;
        }
        out.push(ch);
        used += cw;
    }
    out.push('…');
    used += 1;
    out + &" ".repeat(width.saturating_sub(used))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render(p: Para, width: usize) -> Vec<String> {
        layout(&[p], width)
            .into_iter()
            .map(|l| l.spans.iter().map(|s| s.content.as_ref()).collect())
            .collect()
    }

    #[test]
    fn wraps_words_with_hanging_indent() {
        let p = Para::plain("the quick brown fox jumps over", Style::new())
            .with_prefix(vec![Span::raw("• ")], vec![Span::raw("  ")]);
        assert_eq!(
            render(p, 12),
            vec!["• the quick", "  brown fox", "  jumps over"]
        );
    }

    #[test]
    fn hard_breaks_long_words_and_code() {
        assert_eq!(
            render(Para::plain("abcdefghij", Style::new()), 8),
            vec!["abcdefgh", "ij"]
        );
        assert_eq!(
            render(Para::plain("ab cd ef gh ij", Style::new()).code(), 8),
            vec!["ab cd ef", " gh ij"]
        );
    }

    #[test]
    fn keeps_empty_lines() {
        assert_eq!(render(Para::blank(), 10), vec![""]);
    }

    #[test]
    fn fits() {
        assert_eq!(fit("hello", 7), "hello  ");
        assert_eq!(fit("hello world", 7), "hello …");
    }
}
