//! Builders that turn data into transcript entries.

use ratatui::text::Span;

use super::commands::{COMMANDS, Group, Spec};
use super::transcript::Entry;
use crate::config::{Config, Paths};
use crate::lang::Language;
use crate::questions::{Bank, Question};
use crate::runner::{RunReport, Status, Which};
use crate::session::{Done, Outcome};
use crate::store::{OpenAttempt, QuestionStats};
use crate::ui::markdown;
use crate::ui::text::{Para, fit};
use crate::ui::theme::theme;

fn entry(paras: Vec<Para>) -> Entry {
    let copy = markdown::plain(&paras);
    Entry {
        paras,
        copy,
        input: false,
    }
}

pub fn input(line: &str) -> Entry {
    let t = theme();
    Entry {
        paras: vec![Para::new(vec![
            Span::styled("› ", t.accent_bold()),
            Span::styled(line.to_string(), t.bold()),
        ])],
        copy: String::new(),
        input: true,
    }
}

pub fn info(msg: impl Into<String>) -> Entry {
    let msg = msg.into();
    entry(
        msg.lines()
            .map(|l| Para::plain(l.to_string(), theme().dim()))
            .collect(),
    )
}

pub fn ok(msg: impl Into<String>) -> Entry {
    status("✓ ", msg.into(), theme().ok())
}

pub fn error(msg: impl Into<String>) -> Entry {
    status("✗ ", msg.into(), theme().err())
}

pub fn warn(msg: impl Into<String>) -> Entry {
    status("! ", msg.into(), theme().warn())
}

fn status(icon: &'static str, msg: String, style: ratatui::style::Style) -> Entry {
    let mut lines = msg.lines();
    let mut paras = vec![
        Para::new(vec![Span::raw(
            lines.next().unwrap_or_default().to_string(),
        )])
        .with_prefix(vec![Span::styled(icon, style)], vec![Span::raw("  ")]),
    ];
    paras.extend(lines.map(|l| Para::plain(l.to_string(), theme().dim()).indent(2)));
    entry(paras)
}

pub fn welcome(bank: &Bank, config: &Config, unfinished: &[OpenAttempt]) -> Entry {
    let t = theme();
    let (editor, source) = config.editor();
    let mut paras = vec![
        Para::new(vec![
            Span::styled("道場 dojo", t.accent_bold()),
            Span::styled(format!("  v{}", env!("CARGO_PKG_VERSION")), t.dim()),
        ]),
        Para::plain(
            "Interview practice in your terminal, solved in your editor.",
            t.dim(),
        ),
        Para::blank(),
    ];
    let tips = [
        ("/list", format!("browse {} questions", bank.all().len())),
        ("/list dfs", "filter by tag, company or difficulty".into()),
        ("/show 1", "read a problem".into()),
        ("/help", "every command and shortcut".into()),
    ];
    for (cmd, about) in tips {
        paras.push(
            Para::new(vec![
                Span::styled(fit(cmd, 12), t.accent()),
                Span::styled(about, t.text()),
            ])
            .indent(2),
        );
    }
    if !unfinished.is_empty() {
        paras.push(Para::blank());
        paras.push(Para::plain("Unfinished", t.heading()));
        for (i, o) in unfinished.iter().enumerate() {
            let title = bank
                .get(o.question_id as u32)
                .map(|q| q.meta.title.clone())
                .unwrap_or_else(|| "(question removed)".into());
            // The most recent one is what Enter continues.
            let cmd = if i == 0 {
                "⏎ or /edit".to_string()
            } else {
                format!("/edit {}", o.question_id)
            };
            paras.push(
                Para::new(vec![
                    Span::styled(format!("⏸ #{} ", o.question_id), t.warn()),
                    Span::styled(title, t.bold()),
                    Span::styled(
                        format!(
                            "  {} · {} on the clock · started {}  ·  ",
                            o.language,
                            crate::session::clock(std::time::Duration::from_secs(
                                o.active_secs.max(0) as u64
                            )),
                            ago(&o.started_at)
                        ),
                        t.dim(),
                    ),
                    Span::styled(cmd, t.accent()),
                ])
                .indent(2),
            );
        }
    }
    paras.push(Para::blank());
    paras.push(Para::new(vec![
        Span::styled("editor ", t.dim()),
        Span::raw(editor),
        Span::styled(format!(" ({source})"), t.dim()),
        Span::styled("  ·  language ", t.dim()),
        Span::raw(config.language.clone()),
        Span::styled("  ·  change with /editor and /lang", t.dim()),
    ]));
    Entry {
        paras,
        copy: String::new(),
        input: false,
    }
}

/// Title line, labels and the full statement.
pub fn question(q: &Question) -> Entry {
    let t = theme();
    let m = &q.meta;
    let mut paras = vec![
        Para::new(vec![
            Span::styled(format!("#{} ", m.id), t.dim()),
            Span::styled(m.title.clone(), t.heading()),
        ]),
        labels(q),
    ];
    if !m.companies.is_empty() {
        paras.push(Para::plain(
            format!("asked at {}", m.companies.join(", ")),
            t.dim(),
        ));
    }
    paras.push(Para::plain("─".repeat(48), t.border()));
    paras.extend(markdown::render(&q.statement));
    paras.push(Para::plain("─".repeat(48), t.border()));
    paras.push(Para::new(vec![
        Span::styled(format!("/solve {}", m.id), t.accent()),
        Span::styled(" to start  ·  ", t.dim()),
        Span::styled(format!("/hint {}", m.id), t.accent()),
        Span::styled(
            format!(" for a nudge ({} available)", q.hints.len()),
            t.dim(),
        ),
    ]));
    entry(paras)
}

fn labels(q: &Question) -> Para {
    let t = theme();
    let m = &q.meta;
    let mut spans = vec![
        Span::styled(m.difficulty.to_string(), t.difficulty(m.difficulty)),
        Span::styled(format!("  ·  {} min  ·  ", m.target_minutes), t.dim()),
    ];
    for (i, tag) in m.tags.iter().enumerate() {
        if i > 0 {
            spans.push(Span::styled(" ", t.dim()));
        }
        spans.push(Span::styled(format!("#{tag}"), t.accent()));
    }
    Para::new(spans)
}

pub fn question_list(qs: &[&Question], query: &str) -> Entry {
    let t = theme();
    if qs.is_empty() {
        return warn(format!("no questions match `{query}`"));
    }
    let mut paras = vec![Para::new(vec![Span::styled(
        format!(
            "{:>4}  {}  {}  {}",
            "#",
            fit("title", 38),
            fit("level", 7),
            "tags"
        ),
        t.dim(),
    )])];
    let cont = " ".repeat(4 + 2 + 38 + 2 + 7 + 2);
    for q in qs {
        let m = &q.meta;
        paras.push(
            Para::new(vec![
                Span::styled(format!("{:>4}  ", m.id), t.dim()),
                Span::styled(fit(&m.title, 38), t.bold()),
                Span::raw("  "),
                Span::styled(
                    fit(&m.difficulty.to_string(), 7),
                    t.difficulty(m.difficulty),
                ),
                Span::raw("  "),
                Span::styled(m.tags.join(" "), t.dim()),
            ])
            .with_prefix(vec![], vec![Span::raw(cont.clone())]),
        );
    }
    paras.push(Para::blank());
    paras.push(Para::new(vec![
        Span::styled(
            format!(
                "{} question{}  ·  ",
                qs.len(),
                if qs.len() == 1 { "" } else { "s" }
            ),
            t.dim(),
        ),
        Span::styled("/show <id>", t.accent()),
        Span::styled(" to read one", t.dim()),
    ]));
    entry(paras)
}

/// `in_session` hints are taken one at a time with a bare `/hint`.
pub fn hint(q: &Question, n: usize, in_session: bool) -> Entry {
    let t = theme();
    let h = &q.hints[n - 1];
    let mut paras = vec![Para::new(vec![
        Span::styled(format!("Hint {n}/{}", q.hints.len()), t.warn()),
        Span::styled(format!("  ·  #{} {}", q.meta.id, q.meta.title), t.dim()),
    ])];
    paras.extend(markdown::render(&h.body));
    if n < q.hints.len() {
        let next = if in_session {
            "/hint".to_string()
        } else {
            format!("/hint {} {}", q.meta.id, n + 1)
        };
        paras.push(Para::new(vec![
            Span::styled(next, t.accent()),
            Span::styled(" for the next one", t.dim()),
        ]));
    }
    entry(paras)
}

pub fn solution(q: &Question, lang: Language) -> Entry {
    let t = theme();
    let mut paras = vec![Para::new(vec![
        Span::styled("Solution", t.warn()),
        Span::styled(format!("  ·  #{} {}", q.meta.id, q.meta.title), t.dim()),
    ])];
    paras.extend(markdown::render(&q.explanation));
    // Prefer the user's language; fall back to any reference solution.
    let shown = q
        .solutions
        .get_key_value(&lang)
        .or_else(|| q.solutions.iter().next());
    if let Some((lang, code)) = shown {
        paras.push(Para::blank());
        paras.push(Para::plain(
            format!("Reference solution ({})", lang.label()),
            t.bold(),
        ));
        paras.extend(markdown::render(&format!("```\n{}\n```", code.trim_end())));
    }
    entry(paras)
}

fn command_line(c: &Spec) -> Para {
    let t = theme();
    let usage = if c.args.is_empty() {
        format!("/{}", c.name)
    } else {
        format!("/{} {}", c.name, c.args)
    };
    let mut spans = vec![
        Span::styled(
            fit(&usage, 30),
            if c.soon.is_some() {
                t.dim()
            } else {
                t.accent()
            },
        ),
        Span::raw("  "),
        Span::styled(
            c.about.to_string(),
            if c.soon.is_some() { t.dim() } else { t.text() },
        ),
    ];
    if let Some(m) = c.soon {
        spans.push(Span::styled(format!(" · {m}"), t.dim()));
    }
    Para::new(spans).with_prefix(vec![Span::raw("  ")], vec![Span::raw(" ".repeat(34))])
}

pub fn help(only: Option<&Spec>) -> Entry {
    let t = theme();
    if let Some(c) = only {
        let mut paras = vec![command_line(c)];
        if !c.aliases.is_empty() {
            paras.push(
                Para::plain(
                    format!(
                        "aliases: {}",
                        c.aliases
                            .iter()
                            .map(|a| format!("/{a}"))
                            .collect::<Vec<_>>()
                            .join(", ")
                    ),
                    t.dim(),
                )
                .indent(2),
            );
        }
        return entry(paras);
    }

    let mut paras = Vec::new();
    for group in Group::ORDER {
        paras.push(Para::plain(group.title(), t.heading()));
        paras.extend(
            COMMANDS
                .iter()
                .filter(|c| c.group == *group)
                .map(command_line),
        );
        paras.push(Para::blank());
    }
    paras.push(Para::plain("Keys", t.heading()));
    let keys = [
        (
            "Tab",
            "complete  ·  ↑/↓ choose a suggestion or walk history",
        ),
        (
            "PgUp / PgDn",
            "scroll  ·  Shift+↑/↓ one line  ·  mouse wheel",
        ),
        (
            "Esc",
            "close suggestions, jump to the bottom, then clear input",
        ),
        ("Ctrl+L", "clear the screen"),
        ("Ctrl+C", "clear input  ·  twice on an empty prompt to quit"),
        ("Shift+drag", "select text (most terminals) or use /copy"),
    ];
    for (k, about) in keys {
        paras.push(
            Para::new(vec![
                Span::styled(fit(k, 30), t.accent()),
                Span::raw("  "),
                Span::raw(about),
            ])
            .with_prefix(vec![Span::raw("  ")], vec![Span::raw(" ".repeat(34))]),
        );
    }
    entry(paras)
}

pub fn config(config: &Config, paths: &Paths) -> Entry {
    let t = theme();
    let (editor, source) = config.editor();
    let row = |k: &str, v: String, note: String| {
        Para::new(vec![
            Span::styled(fit(k, 12), t.dim()),
            Span::raw(v),
            Span::styled(note, t.dim()),
        ])
        .with_prefix(vec![Span::raw("  ")], vec![Span::raw(" ".repeat(14))])
    };
    entry(vec![
        Para::plain("Settings", t.heading()),
        row("editor", editor, format!("  ({source})")),
        row("language", config.language.clone(), String::new()),
        row(
            "workspace",
            config.workspace(paths).display().to_string(),
            String::new(),
        ),
        Para::blank(),
        Para::plain("Files", t.heading()),
        row(
            "config",
            paths.config_file.display().to_string(),
            String::new(),
        ),
        row("data", paths.db.display().to_string(), String::new()),
    ])
}

// ---- sessions -------------------------------------------------------------

fn tilde(path: &std::path::Path) -> String {
    let p = path.display().to_string();
    match directories::BaseDirs::new() {
        Some(b) => match p.strip_prefix(&b.home_dir().display().to_string()) {
            Some(rest) => format!("~{rest}"),
            None => p,
        },
        None => p,
    }
}

/// "3 days ago" for an RFC 3339 timestamp.
pub fn ago(ts: &str) -> String {
    let Ok(then) = ts.parse::<jiff::Timestamp>() else {
        return ts.to_string();
    };
    let secs = (jiff::Timestamp::now().as_second() - then.as_second()).max(0);
    match secs {
        s if s < 90 => "just now".into(),
        s if s < 90 * 60 => format!("{} min ago", s / 60),
        s if s < 36 * 3600 => format!("{} h ago", s / 3600),
        s => format!("{} days ago", s / 86400),
    }
}

/// How an attempt's working file came to be.
pub enum Resume {
    /// New attempt, boilerplate.
    Fresh,
    /// An open attempt continued with its clock and counters.
    Continued {
        elapsed: std::time::Duration,
        started_at: String,
    },
}

pub fn session_question(
    q: &Question,
    position: (usize, usize),
    file: &std::path::Path,
    stats: &QuestionStats,
    resume: &Resume,
) -> Entry {
    let t = theme();
    let m = &q.meta;
    let mut title = vec![
        Span::styled(format!("#{} ", m.id), t.dim()),
        Span::styled(m.title.clone(), t.heading()),
    ];
    if position.1 > 1 {
        title.push(Span::styled(
            format!("   question {} of {}", position.0 + 1, position.1),
            t.dim(),
        ));
    }
    let mut paras = vec![Para::new(title), labels(q)];
    if stats.attempts > 0 {
        let mut seen = format!(
            "seen before: {} attempt{}, solved {}×",
            stats.attempts,
            if stats.attempts == 1 { "" } else { "s" },
            stats.solved
        );
        if let Some(best) = stats.best_secs {
            seen += &format!(
                " · best {}",
                crate::session::clock(std::time::Duration::from_secs(best as u64))
            );
        }
        if let Some(last) = &stats.last_at {
            seen += &format!(" · last {}", ago(last));
        }
        paras.push(Para::plain(seen, t.warn()));
    }
    paras.push(Para::plain("─".repeat(48), t.border()));
    paras.extend(markdown::render(&q.statement));
    paras.push(Para::plain("─".repeat(48), t.border()));
    paras.push(Para::new(vec![
        Span::styled("✎ ", t.accent()),
        Span::raw(tilde(file)),
    ]));
    match resume {
        Resume::Fresh => {}
        Resume::Continued {
            elapsed,
            started_at,
        } => paras.push(Para::new(vec![
            Span::styled(
                format!(
                    "↺ continuing your attempt from {}  ·  {} on the clock  ·  ",
                    ago(started_at),
                    crate::session::clock(*elapsed)
                ),
                t.warn(),
            ),
            Span::styled(format!("/solve {}", q.meta.id), t.accent()),
            Span::styled(" starts over instead", t.warn()),
        ])),
    }
    paras.push(Para::new(vec![
        Span::styled("/test", t.accent()),
        Span::styled(" visible tests  ·  ", t.dim()),
        Span::styled("/submit", t.accent()),
        Span::styled(" all tests  ·  ", t.dim()),
        Span::styled("/hint", t.accent()),
        Span::styled(" nudge  ·  ", t.dim()),
        Span::styled("/edit", t.accent()),
        Span::styled(" reopen editor", t.dim()),
    ]));
    entry(paras)
}

fn short(v: &serde_json::Value, max: usize) -> String {
    let s = v.to_string();
    if s.chars().count() > max {
        format!("{}…", s.chars().take(max).collect::<String>())
    } else {
        s
    }
}

fn case_input(q: &Question, index: usize) -> String {
    let case = &q.cases[index];
    q.meta
        .signature
        .params
        .iter()
        .map(|p| format!("{} = {}", p.name, short(&case.input[&p.name], 80)))
        .collect::<Vec<_>>()
        .join("  ·  ")
}

/// Lines of printed output shown per case before eliding.
const PRINT_LINES: usize = 12;

/// Rows showing what a solution printed, labelled "printed".
fn printed(out: &str, label: &str, indent: usize) -> Vec<Para> {
    let t = theme();
    let lines: Vec<&str> = out.trim_end().lines().collect();
    let mut paras: Vec<Para> = lines
        .iter()
        .take(PRINT_LINES)
        .enumerate()
        .map(|(i, l)| {
            Para::new(vec![
                Span::styled(fit(if i == 0 { label } else { "" }, 10), t.warn()),
                Span::styled(l.to_string(), t.text()),
            ])
            .code()
            .with_prefix(
                vec![Span::raw(" ".repeat(indent))],
                vec![Span::raw(" ".repeat(indent + 10))],
            )
        })
        .collect();
    if lines.len() > PRINT_LINES {
        paras.push(
            Para::plain(
                format!("… {} more lines", lines.len() - PRINT_LINES),
                t.dim(),
            )
            .indent(indent + 10),
        );
    }
    paras
}

pub fn test_report(q: &Question, report: &RunReport) -> Entry {
    let t = theme();
    let label = match report.which {
        Which::Visible => "visible tests",
        Which::All => "all tests",
    };
    if let Some(fatal) = &report.fatal {
        let mut paras = vec![Para::new(vec![
            Span::styled("✗ ", t.err()),
            Span::styled(format!("{label}: your solution didn't run"), t.bold()),
        ])];
        if let Some(out) = &report.load_stdout {
            paras.extend(printed(out, "printed", 4));
        }
        paras.extend(
            fatal
                .lines()
                .map(|l| Para::plain(l.to_string(), t.err()).code().indent(4)),
        );
        return entry(paras);
    }

    let (passed, total) = (report.passed(), report.total());
    let ok = report.all_passed();
    let mut paras = vec![Para::new(vec![
        Span::styled(
            if ok { "✓ " } else { "✗ " },
            if ok { t.ok() } else { t.err() },
        ),
        Span::styled(format!("{passed}/{total} {label} passed"), t.bold()),
        Span::styled(format!("  ·  {} ms", report.elapsed.as_millis()), t.dim()),
    ])];
    if let Some(out) = &report.load_stdout {
        paras.push(Para::plain("printed while loading the file:", t.dim()).indent(2));
        paras.extend(printed(out, "printed", 4));
    }

    // `/test` lists every case; `/submit` lists only failures.
    let failures = report
        .cases
        .iter()
        .filter(|c| c.status != Status::Pass)
        .count();
    let mut shown_failures = 0;
    for c in &report.cases {
        let pass = c.status == Status::Pass;
        if pass && report.which == Which::All {
            continue;
        }
        if !pass {
            shown_failures += 1;
            if shown_failures > 3 {
                continue;
            }
        }
        let (icon, style) = match c.status {
            Status::Pass => ("✓", t.ok()),
            Status::Fail => ("✗", t.err()),
            Status::Error => ("!", t.err()),
            Status::Timeout => ("⏱", t.warn()),
        };
        let mut head = vec![
            Span::styled(format!("{icon} "), style),
            Span::styled(format!("case {:<3}", c.index + 1), t.dim()),
        ];
        if c.hidden {
            head.push(Span::styled("(hidden) ", t.dim()));
        }
        if pass {
            head.push(Span::styled(format!("{:.1} ms", c.ms), t.dim()));
            paras.push(Para::new(head).indent(2));
            if let Some(out) = c.stdout.as_deref().filter(|s| !s.trim().is_empty()) {
                paras.extend(printed(out, "printed", 12));
            }
            continue;
        }
        head.push(Span::raw(case_input(q, c.index)));
        paras.push(
            Para::new(head).with_prefix(vec![Span::raw("  ")], vec![Span::raw(" ".repeat(12))]),
        );
        let row = |k: &str, v: String, style| {
            Para::new(vec![
                Span::styled(fit(k, 10), t.dim()),
                Span::styled(v, style),
            ])
            .with_prefix(
                vec![Span::raw(" ".repeat(12))],
                vec![Span::raw(" ".repeat(22))],
            )
        };
        match c.status {
            Status::Fail => {
                paras.push(row(
                    "expected",
                    short(&q.cases[c.index].output, 300),
                    t.ok(),
                ));
                let got = c.got.as_ref().map(|v| short(v, 300)).unwrap_or_default();
                paras.push(row("got", got, t.err()));
            }
            Status::Timeout => paras.push(row(
                "timeout",
                format!("over {}s", crate::runner::CASE_TIMEOUT.as_secs()),
                t.warn(),
            )),
            _ => {
                for l in c.error.as_deref().unwrap_or("error").lines() {
                    paras.push(Para::plain(l.to_string(), t.err()).code().with_prefix(
                        vec![Span::raw(" ".repeat(12))],
                        vec![Span::raw(" ".repeat(12))],
                    ));
                }
            }
        }
        if let Some(out) = c.stdout.as_deref().filter(|s| !s.trim().is_empty()) {
            paras.extend(printed(out, "printed", 12));
        }
    }
    if shown_failures > 3 {
        paras.push(Para::plain(format!("… and {} more failing", failures - 3), t.dim()).indent(2));
    }
    entry(paras)
}

fn outcome_icon(o: Outcome) -> (&'static str, ratatui::style::Style) {
    let t = theme();
    match o {
        Outcome::Pass => ("✓", t.ok()),
        Outcome::Revealed => ("✓", t.warn()),
        Outcome::Fail => ("✗", t.err()),
        Outcome::Skip => ("↷", t.dim()),
        Outcome::Unfinished => ("⏸", t.warn()),
    }
}

pub fn finished(q: &Question, d: &Done, next: Option<&Question>) -> Entry {
    let t = theme();
    let (icon, style) = outcome_icon(d.outcome);
    let verb = match d.outcome {
        Outcome::Pass => "Solved",
        Outcome::Revealed => "Solved (after viewing the solution)",
        Outcome::Fail => "Skipped (not solved)",
        Outcome::Skip => "Skipped",
        Outcome::Unfinished => "Paused",
    };
    let target = std::time::Duration::from_secs(q.meta.target_minutes as u64 * 60);
    let time_style = if d.elapsed <= target {
        t.ok()
    } else {
        t.warn()
    };
    let mut paras = vec![
        Para::new(vec![
            Span::styled(format!("{icon} "), style),
            Span::styled(format!("{verb} #{} {}", q.meta.id, q.meta.title), t.bold()),
        ]),
        Para::new(vec![
            Span::styled(crate::session::clock(d.elapsed), time_style),
            Span::styled(
                format!(" / {} target", crate::session::clock(target)),
                t.dim(),
            ),
            Span::styled(
                format!(
                    "  ·  {} run{} ({} failed)  ·  {} hint{}",
                    d.runs,
                    if d.runs == 1 { "" } else { "s" },
                    d.failed_runs,
                    d.hints,
                    if d.hints == 1 { "" } else { "s" }
                ),
                t.dim(),
            ),
        ])
        .indent(2),
    ];
    let mut tips = vec![];
    if d.outcome != Outcome::Unfinished {
        tips.push(("/solution", " explained solution"));
    }
    match next {
        Some(_) => tips.push(("/next", " next question")),
        None => tips.push(("/quit", " summary & exit")),
    }
    let mut spans = vec![];
    for (i, (cmd, about)) in tips.into_iter().enumerate() {
        if i > 0 {
            spans.push(Span::styled("  ·  ", t.dim()));
        }
        spans.push(Span::styled(cmd, t.accent()));
        spans.push(Span::styled(about, t.dim()));
    }
    if let Some(n) = next {
        spans.push(Span::styled(
            format!(": #{} {}", n.meta.id, n.meta.title),
            t.dim(),
        ));
    }
    paras.push(Para::new(spans).indent(2));
    entry(paras)
}

pub fn session_summary(bank: &Bank, done: &[Done]) -> Entry {
    let t = theme();
    let total: std::time::Duration = done.iter().map(|d| d.elapsed).sum();
    let solved = done
        .iter()
        .filter(|d| matches!(d.outcome, Outcome::Pass | Outcome::Revealed))
        .count();
    let mut paras = vec![Para::new(vec![
        Span::styled("Session complete", t.heading()),
        Span::styled(
            format!(
                "  ·  solved {solved}/{}  ·  {}",
                done.len(),
                crate::session::clock(total)
            ),
            t.dim(),
        ),
    ])];
    for d in done {
        let (icon, style) = outcome_icon(d.outcome);
        let title = bank
            .get(d.question_id)
            .map(|q| q.meta.title.clone())
            .unwrap_or_default();
        paras.push(
            Para::new(vec![
                Span::styled(format!("{icon} "), style),
                Span::styled(format!("#{:<4}", d.question_id), t.dim()),
                Span::raw(fit(&title, 36)),
                Span::styled(fit(d.outcome.as_str(), 11), style),
                Span::raw(crate::session::clock(d.elapsed)),
                Span::styled(format!("  {} runs · {} hints", d.runs, d.hints), t.dim()),
            ])
            .indent(2),
        );
    }
    if done.is_empty() {
        paras.push(Para::plain("no questions attempted", t.dim()).indent(2));
    }
    entry(paras)
}

/// Plain-text session summary printed to the normal terminal on exit.
pub fn summary_text(bank: &Bank, done: &[Done]) -> String {
    let solved = done
        .iter()
        .filter(|d| matches!(d.outcome, Outcome::Pass | Outcome::Revealed))
        .count();
    let mut out = format!("dojo · solved {solved}/{}\n", done.len());
    for d in done {
        let title = bank
            .get(d.question_id)
            .map(|q| q.meta.title.as_str())
            .unwrap_or("");
        out += &format!(
            "  {} #{:<4} {}  {}  {}\n",
            outcome_icon(d.outcome).0,
            d.question_id,
            fit(title, 36),
            fit(d.outcome.as_str(), 10),
            crate::session::clock(d.elapsed)
        );
    }
    if let Some(d) = done.iter().find(|d| d.outcome == Outcome::Unfinished) {
        out += &format!(
            "\n#{} is paused at {}  ·  run dojo and press Enter (/edit) to continue it\n",
            d.question_id,
            crate::session::clock(d.elapsed)
        );
    }
    out.trim_end().to_string()
}

/// Why `/solve need` picked each question.
pub fn need_picks(picks: &[crate::model::Suggestion]) -> Entry {
    let t = theme();
    let mut paras = vec![Para::plain(
        "Picked for where you need practice",
        t.heading(),
    )];
    for (i, p) in picks.iter().enumerate() {
        paras.push(
            Para::new(vec![
                Span::styled(format!("{}. ", i + 1), t.dim()),
                Span::styled(fit(&format!("#{}", p.question_id), 5), t.accent()),
                Span::styled(fit(&p.title, 34), t.bold()),
                Span::styled(p.reason.clone(), t.dim()),
            ])
            .indent(2),
        );
    }
    entry(paras)
}

/// `/past`: every attempt on a question, and the code of one of them.
pub fn past(q: &Question, attempts: &[crate::store::PastAttempt], shown: usize) -> Entry {
    let t = theme();
    let mut paras = vec![Para::new(vec![
        Span::styled("Your attempts", t.heading()),
        Span::styled(format!("  ·  #{} {}", q.meta.id, q.meta.title), t.dim()),
    ])];
    for (i, a) in attempts.iter().enumerate() {
        let n = i + 1;
        let (icon, style) = match a.outcome.as_deref() {
            Some("pass") => ("✓", t.ok()),
            Some("revealed") => ("✓", t.warn()),
            Some("fail") => ("✗", t.err()),
            Some("skip") => ("↷", t.dim()),
            _ => ("⏸", t.warn()),
        };
        let when = a
            .started_at
            .parse::<jiff::Timestamp>()
            .map(|ts| {
                ts.to_zoned(jiff::tz::TimeZone::system())
                    .strftime("%b %-d, %H:%M")
                    .to_string()
            })
            .unwrap_or_default();
        let marker = if n == shown { "▸" } else { " " };
        paras.push(
            Para::new(vec![
                Span::styled(format!("{marker} {n:>2}  "), t.accent()),
                Span::styled(fit(&when, 15), t.dim()),
                Span::raw(fit(&a.language, 11)),
                Span::styled(format!("{icon} "), style),
                Span::styled(fit(a.outcome.as_deref().unwrap_or("unfinished"), 11), style),
                Span::raw(crate::session::clock(std::time::Duration::from_secs(
                    a.active_secs.max(0) as u64,
                ))),
                Span::styled(
                    format!(
                        "  {} run{} · {} hint{}",
                        a.test_runs,
                        if a.test_runs == 1 { "" } else { "s" },
                        a.hints_used,
                        if a.hints_used == 1 { "" } else { "s" }
                    ),
                    t.dim(),
                ),
            ])
            .indent(1),
        );
    }

    let a = &attempts[shown - 1];
    paras.push(Para::blank());
    paras.push(Para::new(vec![
        Span::styled(format!("Attempt {shown}"), t.bold()),
        Span::styled(
            format!(
                "  ·  {}  ·  {}",
                a.language,
                a.outcome.as_deref().unwrap_or("unfinished")
            ),
            t.dim(),
        ),
    ]));
    match a.code.as_deref().filter(|c| !c.trim().is_empty()) {
        Some(code) => paras.extend(markdown::render(&format!("```\n{}\n```", code.trim_end()))),
        None => paras.push(Para::plain("no code was saved for this attempt", t.dim()).indent(2)),
    }
    if attempts.len() > 1 {
        paras.push(Para::new(vec![
            Span::styled(format!("/past {} <n>", q.meta.id), t.accent()),
            Span::styled(" to see another attempt", t.dim()),
        ]));
    }
    entry(paras)
}

pub fn donate(url: &str, opened: bool) -> Entry {
    let t = theme();
    entry(vec![
        Para::plain("Thank you for supporting dojo", t.heading()),
        Para::plain(
            "dojo is free and open source. Donations keep it maintained and the question bank growing.",
            t.text(),
        ),
        Para::blank(),
        Para::new(vec![
            Span::styled(if opened { "opened  " } else { "visit  " }, t.dim()),
            Span::styled(
                url.to_string(),
                t.accent()
                    .add_modifier(ratatui::style::Modifier::UNDERLINED),
            ),
        ]),
    ])
}

// ---- /contribute ------------------------------------------------------------

pub fn contrib_intro(p: crate::contribute::llm::Provider, model: &str) -> Entry {
    let provider = p.label();
    let t = theme();
    entry(vec![
        Para::plain("Contribute a question", t.heading()),
        Para::plain(
            "Describe the question you'd like to add: the problem, and anything about inputs, difficulty or companies that ask it. dojo drafts the whole question (statement, hints, explanation, starter code and reference solutions in Python and JavaScript, tests), runs the solutions to compute the expected outputs, and validates it.",
            t.text(),
        ),
        Para::blank(),
        Para::new(vec![
            Span::styled("Then ", t.dim()),
            Span::styled("/accept", t.accent()),
            Span::styled(
                " opens a pull request, or type what to change and dojo redrafts.",
                t.dim(),
            ),
        ]),
        Para::plain(
            if p.needs_key() {
                format!(
                    "Drafting uses {provider} ({model}) with your API key; your description is sent to {provider}."
                )
            } else {
                format!(
                    "Drafting runs locally with Ollama ({model}): free, and nothing leaves your machine. A draft can take several minutes."
                )
            },
            t.dim(),
        ),
    ])
}

pub fn contrib_review(
    d: &crate::contribute::draft::Draft,
    b: &crate::contribute::draft::Built,
    model: &str,
    rounds: u32,
    usage: crate::contribute::llm::Usage,
) -> Entry {
    let t = theme();
    let ok = b.problems.is_empty();
    let mut paras = vec![Para::new(vec![
        Span::styled(
            if ok { "✓ " } else { "✗ " },
            if ok { t.ok() } else { t.err() },
        ),
        Span::styled(
            if ok {
                "Draft ready"
            } else {
                "Draft has problems"
            },
            t.bold(),
        ),
        Span::styled(
            match rounds {
                0 => "  ·  re-checked".to_string(),
                1 => format!("  ·  {model}"),
                n => format!("  ·  {model}, {n} rounds"),
            },
            t.dim(),
        ),
    ])];
    paras.push(Para::blank());
    paras.push(Para::new(vec![Span::styled(d.title.clone(), t.heading())]));
    paras.push(Para::new(vec![
        Span::styled(d.difficulty.to_string(), t.difficulty(d.difficulty)),
        Span::styled(format!("  ·  {} min  ·  ", d.target_minutes), t.dim()),
        Span::styled(
            d.tags
                .iter()
                .map(|x| format!("#{x}"))
                .collect::<Vec<_>>()
                .join(" "),
            t.accent(),
        ),
    ]));
    if !d.companies.is_empty() {
        paras.push(Para::plain(
            format!("asked at {}", d.companies.join(", ")),
            t.dim(),
        ));
    }
    paras.push(Para::new(vec![
        Span::styled("signature  ", t.dim()),
        Span::styled(super::contribute::signature(d), t.code()),
    ]));
    paras.push(Para::plain("─".repeat(48), t.border()));
    paras.extend(markdown::render(&d.statement));
    paras.push(Para::plain("─".repeat(48), t.border()));

    let check = |good: bool, text: String| {
        Para::new(vec![
            Span::styled(
                if good { "✓ " } else { "✗ " },
                if good { t.ok() } else { t.err() },
            ),
            Span::raw(text),
        ])
        .indent(2)
    };
    paras.push(Para::plain("Checks", t.bold()));
    paras.push(check(
        true,
        format!(
            "{} tests: {} visible, {} hidden{}  ·  expected outputs computed from the Python solution",
            b.visible + b.hidden,
            b.visible,
            b.hidden,
            if b.generated > 0 {
                format!(" ({} generated)", b.generated)
            } else {
                String::new()
            }
        ),
    ));
    if ok {
        paras.push(check(
            true,
            "Python and JavaScript solutions pass every test".into(),
        ));
        paras.push(check(
            true,
            "starter code loads in both languages and doesn't pass".into(),
        ));
        paras.push(check(
            true,
            format!("{} hints, explanation, statement", d.hints.len()),
        ));
    } else {
        for p in &b.problems {
            let mut lines = p.lines();
            paras.push(check(false, lines.next().unwrap_or_default().to_string()));
            for l in lines.take(6) {
                paras.push(Para::plain(l.to_string(), t.dim()).code().indent(6));
            }
        }
    }
    paras.push(Para::blank());
    paras.push(Para::new(vec![
        Span::styled("files  ", t.dim()),
        Span::raw(tilde(&b.dir)),
    ]));
    paras.push(Para::plain(
        format!(
            "tokens so far  {:.1}k in · {:.1}k out",
            usage.input_tokens as f64 / 1000.0,
            usage.output_tokens as f64 / 1000.0
        ),
        t.dim(),
    ));
    entry(paras)
}

pub fn contrib_submitted(url: &str) -> Entry {
    let t = theme();
    entry(vec![
        Para::new(vec![
            Span::styled("✓ ", t.ok()),
            Span::styled("Pull request opened", t.bold()),
        ]),
        Para::new(vec![Span::styled(
            url.to_string(),
            t.accent().add_modifier(ratatui::style::Modifier::UNDERLINED),
        )])
        .indent(2),
        Para::plain(
            "CI re-runs every check; once merged, the question ships in the next dojo release. Thank you!",
            t.dim(),
        )
        .indent(2),
    ])
}
