//! Drawing. Layout, top to bottom: header, transcript, input, status bar.
//! The suggestion popup floats over the bottom of the transcript.

pub mod clipboard;
pub mod markdown;
pub mod report;
pub mod terminal;
pub mod text;
pub mod theme;

use std::time::Duration;

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Margin, Position, Rect};
use ratatui::text::{Line, Span};
use ratatui::widgets::{
    Block, BorderType, Clear, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
};
use unicode_width::UnicodeWidthStr;

use crate::app::App;
use text::fit;
use theme::theme;

const NOTICE_TTL: Duration = Duration::from_secs(3);

pub fn draw(f: &mut Frame, app: &mut App) {
    let [header, body, input, status] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Fill(1),
        Constraint::Length(3),
        Constraint::Length(1),
    ])
    .areas(f.area());

    draw_header(f, app, header);
    if app.report.is_some() {
        let area = Rect {
            height: body.height + input.height,
            ..body
        };
        draw_report(f, app, area);
        draw_report_status(f, status);
        return;
    }
    draw_transcript(f, app, body);
    draw_input(f, app, input);
    draw_status(f, app, status);
    if app.popup_open() {
        draw_popup(f, app, input);
    }
}

fn draw_header(f: &mut Frame, app: &App, area: Rect) {
    let t = theme();
    let left = Line::from(vec![
        Span::styled(" dojo ", t.badge()),
        Span::styled("  interview practice", t.dim()),
    ]);
    let right = format!("{} questions ", app.bank.all().len());
    f.render_widget(Paragraph::new(left), area);
    f.render_widget(
        Paragraph::new(Span::styled(right, t.dim())).right_aligned(),
        area,
    );
}

fn draw_transcript(f: &mut Frame, app: &mut App, area: Rect) {
    let t = theme();
    let text_area = area.inner(Margin::new(1, 0));
    let text_area = Rect {
        width: text_area.width.saturating_sub(1), // scrollbar column
        ..text_area
    };
    let height = text_area.height as usize;
    let total = app.transcript.lines(text_area.width as usize).len();
    app.body_height = height;
    app.body_lines = total;
    let max_scroll = total.saturating_sub(height);
    app.transcript.settle(height);
    let scroll = app.transcript.scroll();

    let end = total - scroll;
    let start = end.saturating_sub(height);
    let visible = app.transcript.lines(text_area.width as usize)[start..end].to_vec();
    f.render_widget(Paragraph::new(visible), text_area);

    if max_scroll > 0 {
        let mut state = ScrollbarState::new(max_scroll).position(max_scroll - scroll);
        f.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(None)
                .end_symbol(None)
                .track_symbol(Some("│"))
                .thumb_symbol("┃")
                .track_style(t.border())
                .thumb_style(t.dim()),
            area,
            &mut state,
        );
    }
    if scroll > 0 {
        let label = format!(" ↓ {scroll} more lines · Esc ");
        let w = label.width() as u16;
        let r = Rect {
            x: area.right().saturating_sub(w + 2),
            y: area.bottom().saturating_sub(1),
            width: w.min(area.width),
            height: 1,
        };
        f.render_widget(Clear, r);
        f.render_widget(Paragraph::new(Span::styled(label, t.selected())), r);
    }
}

fn draw_report(f: &mut Frame, app: &mut App, area: Rect) {
    let t = theme();
    let Some(view) = app.report.as_mut() else {
        return;
    };
    let [bar, _, body] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Fill(1),
    ])
    .areas(area);
    f.render_widget(
        Paragraph::new(report::tab_bar(view)),
        bar.inner(Margin::new(1, 0)),
    );

    let text = body.inner(Margin::new(1, 0));
    let text = Rect {
        width: text.width.saturating_sub(1),
        ..text
    };
    let height = text.height as usize;
    let total = view.lines(text.width as usize).len();
    let max = total.saturating_sub(height);
    view.scroll = view.scroll.min(max);
    let start = view.scroll;
    let visible = view.lines(text.width as usize)[start..(start + height).min(total)].to_vec();
    f.render_widget(Paragraph::new(visible), text);
    app.body_height = height;
    if max > 0 {
        let mut state = ScrollbarState::new(max).position(start);
        f.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(None)
                .end_symbol(None)
                .track_symbol(Some("│"))
                .thumb_symbol("┃")
                .track_style(t.border())
                .thumb_style(t.dim()),
            body,
            &mut state,
        );
    }
}

fn draw_report_status(f: &mut Frame, area: Rect) {
    let t = theme();
    let keys = [
        ("←/→", "tabs"),
        ("1-5", "jump"),
        ("↑/↓ PgUp/PgDn", "scroll"),
        ("Esc", "close"),
    ];
    let mut spans = vec![Span::raw(" ")];
    for (i, (k, what)) in keys.iter().enumerate() {
        if i > 0 {
            spans.push(Span::styled("  ·  ", t.border()));
        }
        spans.push(Span::styled(*k, t.accent()));
        spans.push(Span::styled(format!(" {what}"), t.dim()));
    }
    f.render_widget(Paragraph::new(Line::from(spans)), area);
}

fn draw_input(f: &mut Frame, app: &App, area: Rect) {
    let t = theme();
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(t.border_focus());
    let inner = block.inner(area);
    f.render_widget(block, area);

    let prompt = "› ";
    let avail = (inner.width as usize).saturating_sub(prompt.width() + 1);
    // API keys are masked.
    let masked;
    let (text, cursor) = if app.secret_for.is_some() {
        masked = "•".repeat(app.input.char_count());
        (masked.as_str(), app.input.char_count())
    } else {
        (app.input.text(), app.input.cursor_col())
    };
    // Scroll horizontally so the cursor stays visible.
    let skip_cols = cursor.saturating_sub(avail);
    let mut shown = String::new();
    let mut col = 0;
    for ch in text.chars() {
        let w = unicode_width::UnicodeWidthChar::width(ch).unwrap_or(0);
        if col >= skip_cols && col + w - skip_cols <= avail {
            shown.push(ch);
        }
        col += w;
    }

    let mut spans = vec![Span::styled(prompt, t.accent_bold())];
    if text.is_empty() {
        let prompt = app.prompt();
        if let Some((_, label)) = &prompt.enter {
            spans.push(Span::styled(format!("⏎ {label}"), t.accent()));
        }
        for (key, about) in &prompt.tips {
            if spans.len() > 1 {
                spans.push(Span::styled("  ·  ", t.border()));
            }
            if !key.is_empty() {
                spans.push(Span::styled(key.clone(), t.accent()));
            }
            if !about.is_empty() {
                let gap = if key.is_empty() { "" } else { " " };
                spans.push(Span::styled(format!("{gap}{about}"), t.dim()));
            }
        }
    } else {
        spans.push(Span::raw(shown));
    }
    f.render_widget(Paragraph::new(Line::from(spans)), inner);
    f.set_cursor_position(Position::new(
        inner.x + (prompt.width() + cursor - skip_cols) as u16,
        inner.y,
    ));
}

fn draw_popup(f: &mut Frame, app: &App, input: Rect) {
    let t = theme();
    let items = &app.completions[..app.completions.len().min(crate::app::MAX_ITEMS)];
    let height = items.len() as u16 + 2;
    let area = Rect {
        x: input.x,
        y: input.y.saturating_sub(height),
        width: input.width,
        height: height.min(input.y),
    };
    let label_w = items
        .iter()
        .map(|i| i.label.width())
        .max()
        .unwrap_or(0)
        .max(12)
        + 3;
    let lines: Vec<Line> = items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let selected = i == app.selected;
            let detail_w = (area.width as usize).saturating_sub(label_w + 4);
            let row = format!(
                " {}{} ",
                fit(&item.label, label_w),
                fit(&item.detail, detail_w)
            );
            if selected {
                Line::from(Span::styled(row, t.selected()))
            } else {
                Line::from(vec![
                    Span::styled(format!(" {}", fit(&item.label, label_w)), t.accent()),
                    Span::styled(fit(&item.detail, detail_w), t.dim()),
                ])
            }
        })
        .collect();
    f.render_widget(Clear, area);
    f.render_widget(
        Paragraph::new(lines).block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .border_style(t.border())
                .title_bottom(Span::styled(
                    " Tab complete · ↑↓ choose · Esc close ",
                    t.dim(),
                )),
        ),
        area,
    );
}

const SPINNER: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

fn draw_status(f: &mut Frame, app: &App, area: Rect) {
    let t = theme();
    let (editor, _) = app.config.editor();
    let editor = editor.split_whitespace().next().unwrap_or("").to_string();
    let sep = || Span::styled("  │  ", t.border());

    let mut left = vec![Span::raw(" ")];
    let attempt = app.session.as_ref().and_then(|s| s.attempt.as_ref());
    match (attempt, &app.session) {
        (Some(a), Some(s)) => {
            let q = app.bank.get(a.question_id);
            let target = Duration::from_secs(q.map_or(30, |q| q.meta.target_minutes) as u64 * 60);
            let elapsed = a.timer.elapsed();
            let ratio = elapsed.as_secs_f64() / target.as_secs_f64().max(1.0);
            let style = if a.timer.paused() {
                t.dim()
            } else if ratio >= 1.0 {
                t.err()
            } else if ratio >= 0.8 {
                t.warn()
            } else {
                t.ok()
            };
            let filled = ((ratio.min(1.0)) * 8.0).round() as usize;
            left.push(Span::styled(
                format!(
                    "{} {} / {} ",
                    if a.timer.paused() { "⏸" } else { "⏱" },
                    crate::session::clock(elapsed),
                    crate::session::clock(target)
                ),
                style,
            ));
            left.push(Span::styled("▰".repeat(filled), style));
            left.push(Span::styled("▱".repeat(8 - filled), t.border()));
            left.push(sep());
            left.push(Span::styled(format!("#{}", a.question_id), t.accent()));
            if s.queue.len() > 1 {
                left.push(Span::styled(
                    format!(" ({}/{})", s.index + 1, s.queue.len()),
                    t.dim(),
                ));
            }
            left.push(sep());
            let hints = q.map_or(0, |q| q.hints.len());
            left.push(Span::styled(
                format!("hints {}/{}", a.hints_used, hints),
                t.dim(),
            ));
            left.push(sep());
            if let Some((_, started)) = app.running {
                let frame = (started.elapsed().as_millis() / 80) as usize % SPINNER.len();
                left.push(Span::styled(
                    format!("{} running tests", SPINNER[frame]),
                    t.accent(),
                ));
            } else if let Some((passed, total)) = a.last_run {
                let style = if passed == total { t.ok() } else { t.err() };
                left.push(Span::styled(format!("{passed}/{total}"), style));
                left.push(Span::styled(format!(" · {} runs", a.test_runs), t.dim()));
            } else if a.test_runs > 0 {
                left.push(Span::styled(format!("{} runs", a.test_runs), t.dim()));
            } else {
                left.push(Span::styled("not run yet", t.dim()));
            }
        }
        (None, Some(s)) => {
            let solved = s
                .done
                .iter()
                .filter(|d| {
                    matches!(
                        d.outcome,
                        crate::session::Outcome::Pass | crate::session::Outcome::Revealed
                    )
                })
                .count();
            left.push(Span::styled(
                format!(
                    "session · {}/{} done · {solved} solved",
                    s.done.len(),
                    s.queue.len()
                ),
                t.accent(),
            ));
        }
        _ => {
            left.push(Span::styled(app.config.language.clone(), t.accent()));
            left.push(Span::styled(" · ", t.dim()));
            left.push(Span::styled(editor.clone(), t.accent()));
        }
    }
    let left = Line::from(left);
    let left_w = left.width();
    f.render_widget(Paragraph::new(left), area);

    let right = match &app.notice {
        Some((msg, at)) if at.elapsed() < NOTICE_TTL => Span::styled(format!("{msg} "), t.warn()),
        _ if app.session.is_some() => {
            Span::styled(format!("{} · {editor} ", app.config.language), t.dim())
        }
        _ => Span::styled("/help · PgUp/PgDn scroll · Ctrl+C quit ", t.dim()),
    };
    // Hints yield to the left side when space is short; notices always show,
    // drawn over the end of the left side if they must.
    let is_notice = matches!(&app.notice, Some((_, at)) if at.elapsed() < NOTICE_TTL);
    if is_notice {
        let w = (right.width() as u16 + 1).min(area.width);
        let r = Rect {
            x: area.right() - w,
            width: w,
            ..area
        };
        f.render_widget(Clear, r);
        f.render_widget(Paragraph::new(right).right_aligned(), r);
    } else if left_w + right.width() < area.width as usize {
        f.render_widget(Paragraph::new(right).right_aligned(), area);
    }
}
