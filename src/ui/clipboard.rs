//! Clipboard: native tools when available, OSC 52 otherwise. OSC 52 asks the
//! terminal to set the clipboard, so it works over SSH, and inside tmux with
//! `set -g set-clipboard on`. Some terminals ignore it (GNOME Terminal and
//! other VTE ones), so on a local Linux machine where it would go nowhere
//! dojo says which tool to install instead of claiming it copied.

use std::io::Write;
use std::process::{Command, Stdio};

/// How the text reached the clipboard.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Copied {
    /// A clipboard tool took it (pbcopy, wl-copy, xclip, xsel).
    Native,
    /// Sent to the terminal as OSC 52; whether it lands is up to the terminal.
    Terminal,
}

pub fn copy(text: &str) -> Result<Copied, String> {
    let remote =
        std::env::var_os("SSH_TTY").is_some() || std::env::var_os("SSH_CONNECTION").is_some();
    if !remote {
        let tools: &[&[&str]] = if cfg!(target_os = "macos") {
            &[&["pbcopy"]]
        } else {
            &[
                &["wl-copy"],
                &["xclip", "-selection", "clipboard"],
                &["xsel", "--clipboard", "--input"],
            ]
        };
        for tool in tools {
            if pipe(tool, text).is_ok() {
                return Ok(Copied::Native);
            }
        }
        if cfg!(target_os = "linux") {
            let vte = std::env::var_os("VTE_VERSION").is_some();
            let display = std::env::var_os("DISPLAY").is_some()
                || std::env::var_os("WAYLAND_DISPLAY").is_some();
            if let Some(why) = osc52_unlikely(vte, display) {
                return Err(format!(
                    "{why}  ·  install wl-clipboard (Wayland) or xclip (X11)"
                ));
            }
        }
    }
    osc52(text).map_err(|e| e.to_string())?;
    Ok(Copied::Terminal)
}

/// Why OSC 52 would most likely go nowhere on a local Linux machine with no
/// working clipboard tool: VTE terminals ignore it, and without a display
/// there's no desktop clipboard for it to reach.
fn osc52_unlikely(vte: bool, display: bool) -> Option<&'static str> {
    if vte {
        Some("no clipboard tool found, and this terminal ignores OSC 52")
    } else if !display {
        Some("no clipboard tool found, and no display (DISPLAY / WAYLAND_DISPLAY)")
    } else {
        None
    }
}

fn pipe(cmd: &[&str], text: &str) -> std::io::Result<()> {
    let mut child = Command::new(cmd[0])
        .args(&cmd[1..])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    child.stdin.take().unwrap().write_all(text.as_bytes())?;
    if child.wait()?.success() {
        Ok(())
    } else {
        Err(std::io::Error::other("clipboard tool failed"))
    }
}

/// Plain OSC 52, inside tmux too: tmux passes it on with `set-clipboard on`,
/// while the DCS passthrough form needs `allow-passthrough`, off by default
/// since tmux 3.3.
fn osc52(text: &str) -> std::io::Result<()> {
    let seq = format!("\x1b]52;c;{}\x07", base64(text.as_bytes()));
    let mut out = std::io::stdout();
    out.write_all(seq.as_bytes())?;
    out.flush()
}

fn base64(data: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(data.len().div_ceil(3) * 4);
    for chunk in data.chunks(3) {
        let b = [
            chunk[0],
            *chunk.get(1).unwrap_or(&0),
            *chunk.get(2).unwrap_or(&0),
        ];
        let n = (b[0] as u32) << 16 | (b[1] as u32) << 8 | b[2] as u32;
        for i in 0..4 {
            if i <= chunk.len() {
                out.push(TABLE[(n >> (18 - 6 * i) & 63) as usize] as char);
            } else {
                out.push('=');
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case(b"", "")]
    #[case(b"f", "Zg==")]
    #[case(b"fo", "Zm8=")]
    #[case(b"foo", "Zm9v")]
    #[case(b"dojo", "ZG9qbw==")]
    #[case(b"\xff\x00\x10", "/wAQ")]
    fn encodes_base64(#[case] input: &[u8], #[case] expected: &str) {
        assert_eq!(super::base64(input), expected);
    }

    #[rstest]
    #[case::vte(true, true, true)]
    #[case::vte_without_display(true, false, true)]
    #[case::no_display(false, false, true)]
    #[case::other_terminal(false, true, false)]
    fn knows_when_osc52_wont_land(#[case] vte: bool, #[case] display: bool, #[case] err: bool) {
        assert_eq!(super::osc52_unlikely(vte, display).is_some(), err);
    }
}
