//! Colors. Only the 16 ANSI colors are used so dojo follows the user's
//! terminal palette (light or dark) and works without truecolor support.
//! `NO_COLOR` disables color entirely; bold/dim/italic still apply.

use std::sync::OnceLock;

use ratatui::style::{Color, Modifier, Style};

use crate::questions::Difficulty;

pub struct Theme {
    color: bool,
}

pub fn theme() -> &'static Theme {
    static THEME: OnceLock<Theme> = OnceLock::new();
    THEME.get_or_init(|| Theme {
        color: std::env::var_os("NO_COLOR").is_none_or(|v| v.is_empty()),
    })
}

impl Theme {
    fn fg(&self, c: Color) -> Style {
        if self.color {
            Style::new().fg(c)
        } else {
            Style::new()
        }
    }

    pub fn text(&self) -> Style {
        Style::new()
    }
    pub fn dim(&self) -> Style {
        self.fg(Color::DarkGray)
    }
    pub fn accent(&self) -> Style {
        self.fg(Color::Cyan)
    }
    pub fn accent_bold(&self) -> Style {
        self.accent().add_modifier(Modifier::BOLD)
    }
    pub fn heading(&self) -> Style {
        self.fg(Color::Magenta).add_modifier(Modifier::BOLD)
    }
    pub fn bold(&self) -> Style {
        Style::new().add_modifier(Modifier::BOLD)
    }
    pub fn ok(&self) -> Style {
        self.fg(Color::Green)
    }
    pub fn err(&self) -> Style {
        self.fg(Color::Red)
    }
    pub fn warn(&self) -> Style {
        self.fg(Color::Yellow)
    }
    pub fn code(&self) -> Style {
        self.fg(Color::Yellow)
    }
    pub fn border(&self) -> Style {
        self.fg(Color::DarkGray)
    }
    pub fn border_focus(&self) -> Style {
        self.fg(Color::Cyan)
    }
    pub fn selected(&self) -> Style {
        if self.color {
            Style::new().fg(Color::Black).bg(Color::Cyan)
        } else {
            Style::new().add_modifier(Modifier::REVERSED)
        }
    }
    pub fn badge(&self) -> Style {
        if self.color {
            Style::new()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::new().add_modifier(Modifier::REVERSED | Modifier::BOLD)
        }
    }
    pub fn difficulty(&self, d: Difficulty) -> Style {
        self.fg(match d {
            Difficulty::Easy => Color::Green,
            Difficulty::Medium => Color::Yellow,
            Difficulty::Hard => Color::Red,
        })
    }
}
