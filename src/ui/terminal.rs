//! Terminal setup and teardown: alternate screen, raw mode, mouse, paste.

use std::io::{self, Stdout, stdout};

use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::event::{
    DisableBracketedPaste, DisableMouseCapture, EnableBracketedPaste, EnableMouseCapture,
};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};

pub type Term = Terminal<CrosstermBackend<Stdout>>;

pub fn init() -> io::Result<Term> {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = leave();
        hook(info);
    }));
    enter()?;
    let mut term = Terminal::new(CrosstermBackend::new(stdout()))?;
    term.clear()?;
    Ok(term)
}

pub fn restore() {
    let _ = leave();
}

/// Hands the terminal to a child process (e.g. a terminal editor).
pub fn suspend() -> io::Result<()> {
    leave()
}

/// Takes the terminal back after `suspend`.
pub fn resume(term: &mut Term) -> io::Result<()> {
    enter()?;
    term.clear()
}

fn enter() -> io::Result<()> {
    enable_raw_mode()?;
    execute!(
        stdout(),
        EnterAlternateScreen,
        EnableMouseCapture,
        EnableBracketedPaste
    )
}

fn leave() -> io::Result<()> {
    execute!(
        stdout(),
        DisableBracketedPaste,
        DisableMouseCapture,
        LeaveAlternateScreen
    )?;
    disable_raw_mode()
}
