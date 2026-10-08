//! Getting a local model ready: Ollama installed, its server running, the
//! model downloaded. Each missing piece is offered to the user, never done
//! silently.

use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use serde_json::Value;

fn on_path(program: &str) -> bool {
    std::env::var_os("PATH")
        .is_some_and(|path| std::env::split_paths(&path).any(|dir| dir.join(program).is_file()))
}

pub fn installed() -> bool {
    on_path("ollama")
}

/// Ollama's address: `OLLAMA_HOST` wins over the config.
pub fn base_url(configured: &str) -> String {
    match std::env::var("OLLAMA_HOST") {
        Ok(h) if !h.trim().is_empty() => {
            if h.starts_with("http") {
                h
            } else {
                format!("http://{h}")
            }
        }
        _ => configured.to_string(),
    }
}

/// How to install Ollama here: a description and a shell command to run
/// with the terminal handed over.
pub fn install_plan() -> Option<(String, String)> {
    if on_path("brew") {
        return Some((
            "with Homebrew: brew install ollama".into(),
            "brew install ollama".into(),
        ));
    }
    if cfg!(target_os = "linux") && on_path("curl") {
        let cmd = "curl -fsSL https://ollama.com/install.sh | sh";
        return Some((format!("with Ollama's installer: {cmd}"), cmd.into()));
    }
    None
}

fn http() -> reqwest::blocking::Client {
    reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(3))
        .build()
        .unwrap_or_default()
}

/// Names of the downloaded models, or `None` if the server isn't up.
pub fn models(base: &str) -> Option<Vec<String>> {
    let resp: Value = http()
        .get(format!("{base}/api/tags"))
        .send()
        .ok()?
        .json()
        .ok()?;
    Some(
        resp["models"]
            .as_array()?
            .iter()
            .filter_map(|m| m["name"].as_str().map(str::to_string))
            .collect(),
    )
}

pub fn has_model(base: &str, model: &str) -> bool {
    let want = if model.contains(':') {
        model.to_string()
    } else {
        format!("{model}:latest")
    };
    models(base).is_some_and(|m| m.iter().any(|n| *n == want || n == model))
}

/// Starts `ollama serve` in the background (logging under `log_dir`) and
/// waits for it to answer.
pub fn start_server(base: &str, log_dir: &std::path::Path) -> bool {
    let log: PathBuf = log_dir.join("ollama.log");
    let _ = std::fs::create_dir_all(log_dir);
    let out = std::fs::File::create(&log).ok();
    let mut cmd = Command::new("ollama");
    cmd.arg("serve").stdin(Stdio::null());
    // Its own process group, so Ctrl-C or closing the terminal doesn't stop
    // the server along with dojo.
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        cmd.process_group(0);
    }
    match out.and_then(|f| f.try_clone().ok().map(|g| (f, g))) {
        Some((a, b)) => {
            cmd.stdout(a).stderr(b);
        }
        None => {
            cmd.stdout(Stdio::null()).stderr(Stdio::null());
        }
    }
    let Ok(child) = cmd.spawn() else { return false };
    std::mem::forget(child); // keeps running after dojo exits
    let deadline = Instant::now() + Duration::from_secs(15);
    while Instant::now() < deadline {
        if models(base).is_some() {
            return true;
        }
        std::thread::sleep(Duration::from_millis(300));
    }
    false
}

/// How to update Ollama (a model may need a newer one), restarting the
/// Homebrew service if that's how it runs.
pub fn update_plan() -> Option<(String, String)> {
    if on_path("brew") {
        return Some((
            "with Homebrew: brew upgrade ollama".into(),
            "brew upgrade ollama && if brew services list | grep -q '^ollama .*started'; then brew services restart ollama; fi".into(),
        ));
    }
    if cfg!(target_os = "linux") && on_path("curl") {
        let cmd = "curl -fsSL https://ollama.com/install.sh | sh";
        return Some((format!("with Ollama's installer: {cmd}"), cmd.into()));
    }
    None
}

pub fn pull_command(model: &str) -> String {
    format!("ollama pull '{}'", model.replace('\'', ""))
}
