//! Markdown → `Para`s for the transcript.

use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use ratatui::style::{Modifier, Style};
use ratatui::text::Span;

use super::text::Para;
use super::theme::theme;

struct ListState {
    next: Option<u64>,
}

#[derive(Default)]
struct Renderer {
    out: Vec<Para>,
    spans: Vec<Span<'static>>,
    styles: Vec<Style>,
    lists: Vec<ListState>,
    /// Bullet for the first paragraph of the current list item.
    item_prefix: Option<String>,
    quote: usize,
    code: Option<String>,
}

impl Renderer {
    fn style(&self) -> Style {
        self.styles.last().copied().unwrap_or_default()
    }

    fn push_style(&mut self, f: impl FnOnce(Style) -> Style) {
        let s = f(self.style());
        self.styles.push(s);
    }

    /// Indent of text inside the current list item (0 outside lists).
    fn indent(&self) -> usize {
        self.lists.len() * 3
    }

    fn quote_prefix(&self) -> Vec<Span<'static>> {
        (0..self.quote)
            .map(|_| Span::styled("│ ", theme().dim()))
            .collect()
    }

    fn blank(&mut self) {
        if self
            .out
            .last()
            .is_some_and(|p| !p.spans.is_empty() || !p.prefix.is_empty())
        {
            let mut p = Para::blank();
            p.prefix = self.quote_prefix();
            self.out.push(p);
        }
    }

    fn flush(&mut self) {
        if self.spans.is_empty() && self.item_prefix.is_none() {
            return;
        }
        let spans = std::mem::take(&mut self.spans);
        let indent = self.indent();
        let (first, cont) = match self.item_prefix.take() {
            Some(bullet) => {
                let base = indent.saturating_sub(3);
                let hang = base + bullet.chars().count();
                (
                    vec![
                        Span::raw(" ".repeat(base)),
                        Span::styled(bullet, theme().accent()),
                    ],
                    vec![Span::raw(" ".repeat(hang))],
                )
            }
            None => (
                vec![Span::raw(" ".repeat(indent))],
                vec![Span::raw(" ".repeat(indent))],
            ),
        };
        let mut prefix = self.quote_prefix();
        prefix.extend(first);
        let mut cont_all = self.quote_prefix();
        cont_all.extend(cont);
        self.out
            .push(Para::new(spans).with_prefix(prefix, cont_all));
    }

    fn event(&mut self, ev: Event) {
        let t = theme();
        match ev {
            Event::Start(tag) => match tag {
                Tag::Paragraph => {}
                Tag::Heading { level, .. } => {
                    self.blank();
                    let style = match level {
                        HeadingLevel::H1 => t.heading(),
                        HeadingLevel::H2 => t.accent_bold(),
                        _ => t.bold(),
                    };
                    self.styles.push(style);
                }
                Tag::BlockQuote(_) => {
                    self.blank();
                    self.quote += 1;
                    self.push_style(|s| s.add_modifier(Modifier::ITALIC));
                }
                Tag::CodeBlock(kind) => {
                    self.flush();
                    self.blank();
                    let lang = match kind {
                        CodeBlockKind::Fenced(l) => l.to_string(),
                        CodeBlockKind::Indented => String::new(),
                    };
                    self.code = Some(lang);
                }
                Tag::List(start) => {
                    self.flush();
                    if self.lists.is_empty() {
                        self.blank();
                    }
                    self.lists.push(ListState { next: start });
                }
                Tag::Item => {
                    self.flush();
                    let list = self.lists.last_mut();
                    let bullet = match list.and_then(|l| {
                        let n = l.next?;
                        l.next = Some(n + 1);
                        Some(n)
                    }) {
                        Some(n) => format!("{n}. "),
                        None => "• ".to_string(),
                    };
                    self.item_prefix = Some(bullet);
                }
                Tag::Emphasis => self.push_style(|s| s.add_modifier(Modifier::ITALIC)),
                Tag::Strong => self.push_style(|s| s.add_modifier(Modifier::BOLD)),
                Tag::Strikethrough => self.push_style(|s| s.add_modifier(Modifier::CROSSED_OUT)),
                Tag::Link { .. } => {
                    let accent = t.accent();
                    self.push_style(|s| s.patch(accent).add_modifier(Modifier::UNDERLINED));
                }
                _ => {}
            },
            Event::End(tag) => match tag {
                TagEnd::Paragraph => {
                    self.flush();
                    if self.lists.is_empty() {
                        self.blank_after();
                    }
                }
                TagEnd::Heading(_) => {
                    self.styles.pop();
                    self.flush();
                    self.blank_after();
                }
                TagEnd::BlockQuote(_) => {
                    self.flush();
                    self.styles.pop();
                    self.quote -= 1;
                    self.blank_after();
                }
                TagEnd::CodeBlock => {
                    self.code = None;
                    self.blank_after();
                }
                TagEnd::List(_) => {
                    self.flush();
                    self.lists.pop();
                    if self.lists.is_empty() {
                        self.blank_after();
                    }
                }
                TagEnd::Item => self.flush(),
                TagEnd::Emphasis | TagEnd::Strong | TagEnd::Strikethrough | TagEnd::Link => {
                    self.styles.pop();
                }
                _ => {}
            },
            Event::Text(text) => {
                if self.code.is_some() {
                    let indent = " ".repeat(self.indent());
                    for line in text.trim_end_matches('\n').split('\n') {
                        let mut prefix = self.quote_prefix();
                        prefix.push(Span::raw(indent.clone()));
                        prefix.push(Span::styled("  │ ", t.dim()));
                        let mut cont = self.quote_prefix();
                        cont.push(Span::raw(indent.clone()));
                        cont.push(Span::styled("  │ ", t.dim()));
                        self.out.push(
                            Para::plain(line.to_string(), t.code())
                                .with_prefix(prefix, cont)
                                .code(),
                        );
                    }
                } else {
                    self.spans
                        .push(Span::styled(text.to_string(), self.style()));
                }
            }
            Event::Code(code) => {
                self.spans
                    .push(Span::styled(code.to_string(), self.style().patch(t.code())));
            }
            Event::SoftBreak => self.spans.push(Span::styled(" ", self.style())),
            Event::HardBreak => self.flush(),
            Event::Rule => {
                self.flush();
                self.out.push(Para::plain("─".repeat(40), t.dim()));
                self.blank_after();
            }
            Event::Html(html) | Event::InlineHtml(html) => {
                self.spans.push(Span::styled(html.to_string(), t.dim()));
            }
            _ => {}
        }
    }

    fn blank_after(&mut self) {
        let mut p = Para::blank();
        p.prefix = self.quote_prefix();
        self.out.push(p);
    }
}

pub fn render(md: &str) -> Vec<Para> {
    let mut r = Renderer::default();
    for ev in Parser::new_ext(md, Options::ENABLE_STRIKETHROUGH) {
        r.event(ev);
    }
    r.flush();
    // Collapse runs of blank lines and trim the ends.
    let mut out: Vec<Para> = Vec::with_capacity(r.out.len());
    for p in r.out {
        let blank = p.spans.is_empty();
        if blank && out.last().is_none_or(|l: &Para| l.spans.is_empty()) {
            continue;
        }
        out.push(p);
    }
    while out.last().is_some_and(|p| p.spans.is_empty()) {
        out.pop();
    }
    out
}

/// Markdown with formatting stripped, for `/copy`.
pub fn plain(paras: &[Para]) -> String {
    paras
        .iter()
        .map(|p| {
            let prefix: String = p.prefix.iter().map(|s| s.content.as_ref()).collect();
            format!("{prefix}{}", p.plain_text())
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    fn text(md: &str) -> Vec<String> {
        render(md)
            .iter()
            .map(|p| {
                let prefix: String = p.prefix.iter().map(|s| s.content.as_ref()).collect();
                format!("{prefix}{}", p.plain_text())
            })
            .collect()
    }

    #[rstest]
    #[case::blocks(
        "# Title\n\nSome `code` here.\n\n- one\n- two\n\n```\na = 1\n```\n",
        &["Title", "", "Some code here.", "", "• one", "• two", "", "  │ a = 1"]
    )]
    #[case::ordered_list("1. a\n2. b\n", &["1. a", "2. b"])]
    #[case::ordered_from_three("3. c\n4. d\n", &["3. c", "4. d"])]
    #[case::soft_break_joins("one\ntwo\n", &["one two"])]
    #[case::emphasis_is_plain("*a* and **b**\n", &["a and b"])]
    #[case::quote("> said\n", &["│ said"])]
    #[case::nested_list("- a\n  - b\n", &["• a", "   • b"])]
    #[case::collapses_blank_lines("a\n\n\n\nb\n", &["a", "", "b"])]
    #[case::empty("", &[])]
    fn renders(#[case] md: &str, #[case] expected: &[&str]) {
        assert_eq!(text(md), expected);
    }
}
