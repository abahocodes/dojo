//! `/report`: a fullscreen, tabbed view of the competence model.

use std::time::Duration;

use jiff::Timestamp;
use jiff::civil::Weekday;
use jiff::tz::TimeZone;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};

use super::text::{Para, fit, layout};
use super::theme::theme;
use crate::model::{GAP, LabelStat, QuestionStat, Report};
use crate::session::{Outcome, clock};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Overview,
    Topics,
    Companies,
    Questions,
    History,
}

impl Tab {
    pub const ALL: [Tab; 5] = [
        Tab::Overview,
        Tab::Topics,
        Tab::Companies,
        Tab::Questions,
        Tab::History,
    ];

    pub fn title(self) -> &'static str {
        match self {
            Tab::Overview => "Overview",
            Tab::Topics => "Topics",
            Tab::Companies => "Companies",
            Tab::Questions => "Questions",
            Tab::History => "History",
        }
    }

    pub fn index(self) -> usize {
        Tab::ALL.iter().position(|t| *t == self).unwrap_or(0)
    }

    pub fn step(self, delta: isize) -> Tab {
        let n = Tab::ALL.len() as isize;
        Tab::ALL[(self.index() as isize + delta).rem_euclid(n) as usize]
    }
}

/// Report state while it's open.
pub struct ReportView {
    pub report: Report,
    pub tab: Tab,
    /// Questions tab filter: a tag, company or difficulty.
    pub filter: Option<String>,
    pub scroll: usize,
    cache: Option<(usize, Tab, Vec<Line<'static>>)>,
}

impl ReportView {
    pub fn new(report: Report, filter: Option<String>) -> ReportView {
        ReportView {
            tab: if filter.is_some() {
                Tab::Questions
            } else {
                Tab::Overview
            },
            report,
            filter,
            scroll: 0,
            cache: None,
        }
    }

    pub fn set_tab(&mut self, tab: Tab) {
        if tab != self.tab {
            self.tab = tab;
            self.scroll = 0;
        }
    }

    pub fn lines(&mut self, width: usize) -> &[Line<'static>] {
        if self
            .cache
            .as_ref()
            .is_none_or(|(w, t, _)| *w != width || *t != self.tab)
        {
            let paras = match self.tab {
                Tab::Overview => overview(&self.report),
                Tab::Topics => labels(&self.report.topics, "Topics", "mastery"),
                Tab::Companies => labels(&self.report.companies, "Companies", "ready"),
                Tab::Questions => questions(&self.report, self.filter.as_deref()),
                Tab::History => history(&self.report),
            };
            self.cache = Some((width, self.tab, layout(&paras, width)));
        }
        &self.cache.as_ref().unwrap().2
    }
}

pub fn tab_bar(view: &ReportView) -> Line<'static> {
    let t = theme();
    let mut spans = vec![Span::styled(" Report ", t.badge()), Span::raw("  ")];
    for (i, tab) in Tab::ALL.iter().enumerate() {
        let style = if *tab == view.tab {
            t.selected()
        } else {
            t.dim()
        };
        spans.push(Span::styled(format!(" {} {} ", i + 1, tab.title()), style));
        spans.push(Span::raw(" "));
    }
    if let (Some(f), Tab::Questions) = (&view.filter, view.tab) {
        spans.push(Span::styled(format!("  filtered: {f}"), t.warn()));
    }
    Line::from(spans)
}

// ---- pieces --------------------------------------------------------------

fn pct(v: f64) -> String {
    format!("{:>3.0}%", v * 100.0)
}

fn level(v: f64) -> Style {
    let t = theme();
    if v >= 0.8 {
        t.ok()
    } else if v >= GAP {
        t.warn()
    } else {
        t.err()
    }
}

fn bar(v: f64, width: usize) -> Vec<Span<'static>> {
    let t = theme();
    let filled = ((v.clamp(0.0, 1.0)) * width as f64).round() as usize;
    vec![
        Span::styled("▰".repeat(filled), level(v)),
        Span::styled("▱".repeat(width - filled), t.border()),
    ]
}

fn confidence(c: u8) -> Span<'static> {
    let dots: String = (0..3).map(|i| if i <= c { '●' } else { '○' }).collect();
    Span::styled(dots, theme().dim())
}

fn heading(s: &str) -> Para {
    Para::plain(s.to_string(), theme().heading())
}

fn hours(secs: u64) -> String {
    let m = secs / 60;
    if m < 60 {
        format!("{m}m")
    } else {
        format!("{}h {:02}m", m / 60, m % 60)
    }
}

fn local(t: Timestamp) -> jiff::Zoned {
    t.to_zoned(TimeZone::system())
}

fn ago(t: Timestamp) -> String {
    crate::app::views_ago(t)
}

fn empty_note(paras: &mut Vec<Para>) {
    paras.push(Para::blank());
    paras.push(Para::new(vec![
        Span::styled("No attempts yet.  ", theme().dim()),
        Span::styled("/solve 1", theme().accent()),
        Span::styled(" to start; this fills in as you practice.", theme().dim()),
    ]));
}

// ---- tabs ----------------------------------------------------------------

fn overview(r: &Report) -> Vec<Para> {
    let t = theme();
    let o = &r.overview;
    let mut p = vec![Para::new(vec![
        Span::styled(format!("{}", o.solved), t.accent_bold()),
        Span::styled(format!("/{} solved", o.questions), t.dim()),
        Span::styled("   ·   ", t.border()),
        Span::styled(format!("{}", o.attempts), t.bold()),
        Span::styled(" attempts", t.dim()),
        Span::styled("   ·   ", t.border()),
        Span::styled(
            o.pass_rate
                .map_or("—".into(), |r| format!("{:.0}%", r * 100.0)),
            t.bold(),
        ),
        Span::styled(" pass rate", t.dim()),
        Span::styled("   ·   ", t.border()),
        Span::styled(hours(o.practice_secs), t.bold()),
        Span::styled(" practiced", t.dim()),
        Span::styled("   ·   ", t.border()),
        Span::styled(
            format!("{}", o.streak_days),
            t.warn().add_modifier(Modifier::BOLD),
        ),
        Span::styled(" day streak", t.dim()),
    ])];
    if o.attempts == 0 && o.practice_secs == 0 {
        empty_note(&mut p);
        return p;
    }

    p.push(Para::blank());
    p.push(heading("Activity"));
    p.extend(heatmap(r));

    p.push(Para::blank());
    p.push(heading("By difficulty"));
    for d in &o.difficulties {
        let frac = if d.total > 0 {
            d.solved as f64 / d.total as f64
        } else {
            0.0
        };
        let mut spans = vec![Span::styled(
            fit(&d.difficulty.to_string(), 8),
            t.difficulty(d.difficulty),
        )];
        spans.extend(bar(frac, 10));
        spans.push(Span::styled(
            format!("  {}/{} solved", d.solved, d.total),
            t.dim(),
        ));
        if let Some(ratio) = d.time_ratio {
            let style = if ratio <= 1.0 { t.ok() } else { t.warn() };
            spans.push(Span::styled("   ·   avg ", t.dim()));
            spans.push(Span::styled(format!("{ratio:.1}×"), style));
            spans.push(Span::styled(" target time", t.dim()));
        }
        p.push(Para::new(spans).indent(2));
    }

    let gaps: Vec<&LabelStat> = r
        .topics
        .iter()
        .filter(|t| t.attempted > 0 && t.mastery < GAP)
        .take(4)
        .collect();
    p.push(Para::blank());
    p.push(heading("Gaps"));
    if gaps.is_empty() {
        p.push(Para::plain("none so far: every practiced topic is above 60%", t.dim()).indent(2));
    }
    for g in gaps {
        p.push(label_row(g, "mastery"));
    }
    let untouched = r.topics.iter().filter(|t| t.attempted == 0).count();
    if untouched > 0 {
        p.push(
            Para::plain(
                format!(
                    "{untouched} topic{} not practiced yet  ·  Topics tab",
                    if untouched == 1 { "" } else { "s" }
                ),
                t.dim(),
            )
            .indent(2),
        );
    }

    if !r.suggestions.is_empty() {
        p.push(Para::blank());
        p.push(heading("Suggested next"));
        for s in &r.suggestions {
            p.push(
                Para::new(vec![
                    Span::styled(fit(&format!("/solve {}", s.question_id), 11), t.accent()),
                    Span::styled(fit(&s.title, 34), t.bold()),
                    Span::styled(s.reason.clone(), t.dim()),
                ])
                .indent(2),
            );
        }
    }
    p
}

/// GitHub-style grid: one column per week, rows Monday to Sunday.
fn heatmap(r: &Report) -> Vec<Para> {
    let t = theme();
    let Some(first) = r.activity.first().map(|(d, _)| *d) else {
        return vec![];
    };
    // Align columns on Mondays.
    let lead = first.weekday().to_monday_zero_offset() as usize;
    let mut cells: Vec<Option<(jiff::civil::Date, u32)>> = vec![None; lead];
    cells.extend(r.activity.iter().map(|c| Some(*c)));
    let weeks = cells.len().div_ceil(7);

    // Month labels above the first column of each month.
    let mut header = vec![Span::raw("      ")];
    let mut last_month = 0;
    let mut pending = String::new();
    for w in 0..weeks {
        let month = cells[w * 7..((w + 1) * 7).min(cells.len())]
            .iter()
            .flatten()
            .find(|(d, _)| d.day() <= 7)
            .map(|(d, _)| d.month());
        let label = match month {
            Some(m) if m != last_month => {
                last_month = m;
                jiff::civil::date(2000, m, 1).strftime("%b").to_string()
            }
            _ => String::new(),
        };
        if !label.is_empty() {
            pending = label;
        }
        if pending.is_empty() {
            header.push(Span::raw("  "));
        } else {
            let take: String = pending.chars().take(2).collect();
            pending = pending.chars().skip(2).collect();
            header.push(Span::styled(fit(&take, 2), t.dim()));
        }
    }
    let mut out = vec![Para::new(header).code()];

    for (row, day) in [
        Weekday::Monday,
        Weekday::Tuesday,
        Weekday::Wednesday,
        Weekday::Thursday,
        Weekday::Friday,
        Weekday::Saturday,
        Weekday::Sunday,
    ]
    .iter()
    .enumerate()
    {
        let name = if row % 2 == 0 {
            format!("{:<4}", &format!("{day:?}")[..3])
        } else {
            "    ".into()
        };
        let mut spans = vec![Span::raw("  "), Span::styled(name, t.dim())];
        for w in 0..weeks {
            let span = match cells.get(w * 7 + row).copied().flatten() {
                None => Span::raw("  "),
                Some((_, 0)) => Span::styled("· ", t.border()),
                Some((_, 1)) => Span::styled("▪ ", t.ok()),
                Some((_, 2)) => Span::styled("■ ", t.ok()),
                Some(_) => Span::styled("■ ", t.ok().add_modifier(Modifier::BOLD)),
            };
            spans.push(span);
        }
        out.push(Para::new(spans).code());
    }
    out.push(
        Para::new(vec![
            Span::styled("less ", t.dim()),
            Span::styled("· ", t.border()),
            Span::styled("▪ ", t.ok()),
            Span::styled("■ ", t.ok()),
            Span::styled("■", t.ok().add_modifier(Modifier::BOLD)),
            Span::styled(" more  ·  attempts per day", t.dim()),
        ])
        .indent(6),
    );
    out
}

fn label_row(l: &LabelStat, word: &str) -> Para {
    let t = theme();
    let mut spans = vec![Span::styled(fit(&l.label, 22), t.bold())];
    if l.attempted == 0 && word == "mastery" {
        spans.push(Span::styled("▱".repeat(10), t.border()));
        spans.push(Span::styled("    —", t.dim()));
    } else {
        spans.extend(bar(l.mastery, 10));
        spans.push(Span::styled(
            format!("  {}", pct(l.mastery)),
            level(l.mastery),
        ));
    }
    spans.push(Span::styled(format!(" {word}   "), t.dim()));
    spans.push(confidence(l.confidence));
    spans.push(Span::styled(
        format!("   {}/{} tried", l.attempted, l.total),
        t.dim(),
    ));
    Para::new(spans).indent(2)
}

fn labels(list: &[LabelStat], title: &str, word: &str) -> Vec<Para> {
    let t = theme();
    let mut p = vec![Para::new(vec![
        Span::styled(title.to_string(), t.heading()),
        Span::styled(
            if word == "ready" {
                "   readiness counts every question the company asks; untried ones count as 0"
            } else {
                "   weakest first  ·  ●●● = how much evidence  ·  /report <name> for its questions"
            },
            t.dim(),
        ),
    ])];
    p.push(Para::blank());
    let mut shown_divider = false;
    for l in list {
        if word == "mastery" && l.attempted == 0 && !shown_divider {
            shown_divider = true;
            p.push(Para::blank());
            p.push(Para::plain("Not practiced yet", t.heading()));
        }
        p.push(label_row(l, word));
    }
    p
}

fn status(q: &QuestionStat) -> Span<'static> {
    let t = theme();
    match (q.solved, q.attempts) {
        (true, _) => Span::styled("✓ ", t.ok()),
        (false, 0) => Span::styled("· ", t.border()),
        _ => Span::styled("✗ ", t.err()),
    }
}

fn questions(r: &Report, filter: Option<&str>) -> Vec<Para> {
    let t = theme();
    let rows: Vec<&QuestionStat> = r
        .questions
        .iter()
        .filter(|q| {
            filter.is_none_or(|f| {
                q.tags.iter().any(|x| x == f)
                    || q.companies.contains(f)
                    || q.difficulty.to_string() == f
            })
        })
        .collect();
    let solved = rows.iter().filter(|q| q.solved).count();
    let mut p = vec![Para::new(vec![
        Span::styled(
            match filter {
                Some(f) => format!("Questions · {f}"),
                None => "Questions".into(),
            },
            t.heading(),
        ),
        Span::styled(format!("   {solved}/{} solved", rows.len()), t.dim()),
    ])];
    p.push(Para::blank());
    p.push(Para::new(vec![Span::styled(
        format!(
            "    {:>4}  {}  {}  {}  {}  {}",
            "#",
            fit("title", 34),
            fit("level", 7),
            fit("score", 12),
            fit("best", 13),
            "last"
        ),
        t.dim(),
    )]));
    for q in rows {
        let mut spans = vec![
            Span::raw("  "),
            status(q),
            Span::styled(format!("{:>4}  ", q.id), t.dim()),
            Span::styled(fit(&q.title, 34), t.bold()),
            Span::raw("  "),
            Span::styled(
                fit(&q.difficulty.to_string(), 7),
                t.difficulty(q.difficulty),
            ),
            Span::raw("  "),
        ];
        match q.score {
            Some(s) => {
                spans.extend(bar(s, 6));
                spans.push(Span::styled(format!(" {}  ", pct(s)), level(s)));
            }
            None => spans.push(Span::styled(fit("", 14), t.dim())),
        }
        let best = q.best_secs.map_or("—".to_string(), |b| {
            format!(
                "{} ({}m)",
                clock(Duration::from_secs(b)),
                q.target_secs / 60
            )
        });
        spans.push(Span::styled(fit(&best, 13), t.dim()));
        spans.push(Span::styled(
            q.last_attempt.map_or(String::new(), |l| {
                format!(
                    "  {} · {}",
                    ago(l),
                    q.last_outcome.map_or("", Outcome::as_str)
                )
            }),
            t.dim(),
        ));
        p.push(Para::new(spans).code());
    }
    p
}

fn history(r: &Report) -> Vec<Para> {
    let t = theme();
    let mut p = vec![heading("Sessions"), Para::blank()];
    if r.sessions.is_empty() {
        empty_note(&mut p);
        return p;
    }
    for s in r.sessions.iter().take(30) {
        let solved = s
            .attempts
            .iter()
            .filter(|a| matches!(a.outcome, Some(Outcome::Pass | Outcome::Revealed)))
            .count();
        p.push(Para::new(vec![
            Span::styled(
                local(s.started_at).strftime("%a %b %-d, %H:%M").to_string(),
                t.bold(),
            ),
            Span::styled(
                format!(
                    "   {} practiced  ·  {} question{}  ·  {solved} solved",
                    hours(s.practice_secs),
                    s.attempts.len(),
                    if s.attempts.len() == 1 { "" } else { "s" }
                ),
                t.dim(),
            ),
        ]));
        for a in &s.attempts {
            let (icon, style) = match a.outcome {
                Some(Outcome::Pass) => ("✓", t.ok()),
                Some(Outcome::Revealed) => ("✓", t.warn()),
                Some(Outcome::Fail) => ("✗", t.err()),
                Some(Outcome::Skip) => ("↷", t.dim()),
                _ => ("⏸", t.warn()),
            };
            p.push(
                Para::new(vec![
                    Span::styled(format!("{icon} "), style),
                    Span::styled(format!("#{:<4}", a.question_id), t.dim()),
                    Span::raw(fit(&a.title, 34)),
                    Span::styled(
                        fit(a.outcome.map_or("unfinished", Outcome::as_str), 11),
                        style,
                    ),
                    Span::styled(clock(Duration::from_secs(a.active_secs)), t.dim()),
                ])
                .indent(2),
            );
        }
        p.push(Para::blank());
    }
    p
}
