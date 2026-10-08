//! Clipboard: native tools when available, OSC 52 otherwise (works over SSH
//! and inside tmux with `set -g set-clipboard on`).

use std::io::Write;
use std::process::{Command, Stdio};

pub fn copy(text: &str) -> Result<(), String> {
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
                return Ok(());
            }
        }
    }
    osc52(text).map_err(|e| e.to_string())
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

fn osc52(text: &str) -> std::io::Result<()> {
    let seq = format!("\x1b]52;c;{}\x07", base64(text.as_bytes()));
    let seq = if std::env::var_os("TMUX").is_some() {
        format!("\x1bPtmux;{}\x1b\\", seq.replace('\x1b', "\x1b\x1b"))
    } else {
        seq
    };
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
}
