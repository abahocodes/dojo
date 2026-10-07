//! Project links shown in the app.

/// Where `/donate` sends people. Keep in sync with `.github/FUNDING.yml`.
/// `None` makes `/donate` say donations aren't set up.
pub const DONATE_URL: Option<&str> = Some("https://github.com/sponsors/abahocodes");

/// Opens a URL in the default browser without blocking.
pub fn open_url(url: &str) -> std::io::Result<()> {
    let program = if cfg!(target_os = "macos") {
        "open"
    } else {
        "xdg-open"
    };
    let mut child = std::process::Command::new(program)
        .arg(url)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()?;
    std::thread::spawn(move || child.wait());
    Ok(())
}
