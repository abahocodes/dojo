//! User config (`config.toml`) and filesystem locations.
//!
//! XDG layout on every platform (`~/.config/dojo`, `~/.local/share/dojo`,
//! `~/.local/state/dojo`), honoring the `XDG_*_HOME` variables. `DOJO_HOME` puts everything
//! under one directory, which is handy for tests and trying things out.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::lang::Language;

#[derive(Debug, Clone, bon::Builder)]
pub struct Paths {
    pub config_file: PathBuf,
    pub db: PathBuf,
    /// Scratch files for attempts in progress (`$XDG_STATE_HOME/dojo/work`).
    pub default_workspace: PathBuf,
}

impl Paths {
    pub fn detect() -> Result<Paths> {
        let home = directories::BaseDirs::new()
            .context("cannot determine home directory")?
            .home_dir()
            .to_path_buf();
        if let Some(root) = std::env::var_os("DOJO_HOME").map(PathBuf::from) {
            return Ok(Paths::builder()
                .config_file(root.join("config.toml"))
                .db(root.join("dojo.db"))
                .default_workspace(root.join("state/work"))
                .build());
        }
        let xdg = |var: &str, fallback: &str| {
            std::env::var_os(var)
                .map(PathBuf::from)
                .filter(|p| p.is_absolute())
                .unwrap_or_else(|| home.join(fallback))
        };
        Ok(Paths::builder()
            .config_file(xdg("XDG_CONFIG_HOME", ".config").join("dojo/config.toml"))
            .db(xdg("XDG_DATA_HOME", ".local/share").join("dojo/dojo.db"))
            .default_workspace(xdg("XDG_STATE_HOME", ".local/state").join("dojo/work"))
            .build())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Editor command. `{file}` and `{dir}` are substituted; when absent the
    /// file path is appended. Falls back to `$VISUAL`, then `$EDITOR`.
    pub editor: Option<String>,
    /// Default solve language.
    pub language: String,
    /// Where working files for attempts live. Defaults to
    /// `~/.local/state/dojo/work`; set it to keep them somewhere you like,
    /// e.g. your own practice repo.
    pub workspace: Option<PathBuf>,
    /// Re-run visible tests whenever the solution file is saved.
    pub auto_test: bool,
    /// `/contribute` settings.
    pub contribute: ContributeConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ContributeConfig {
    /// `claude` or `openai`.
    pub provider: String,
    pub claude_model: String,
    pub openai_model: String,
    pub ollama_model: String,
    /// Where Ollama listens; `OLLAMA_HOST` overrides.
    pub ollama_url: String,
}

impl Default for ContributeConfig {
    fn default() -> Self {
        ContributeConfig {
            provider: "claude".into(),
            claude_model: "claude-opus-5-5".into(),
            openai_model: "gpt-5".into(),
            ollama_model: "qwen3.8:27b".into(),
            ollama_url: "http://localhost:11434".into(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            editor: None,
            language: "python".into(),
            workspace: None,
            auto_test: true,
            contribute: ContributeConfig::default(),
        }
    }
}

impl Config {
    pub fn load(path: &Path) -> Result<Config> {
        match std::fs::read_to_string(path) {
            Ok(s) => toml::from_str(&s).with_context(|| format!("invalid {}", path.display())),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Config::default()),
            Err(e) => Err(e).with_context(|| format!("reading {}", path.display())),
        }
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        let body = format!(
            "# dojo config — edit freely or use /editor, /lang in the app.\n\n{}",
            toml::to_string_pretty(self)?
        );
        let tmp = path.with_extension("toml.tmp");
        std::fs::write(&tmp, body)?;
        std::fs::rename(&tmp, path)?;
        Ok(())
    }

    /// The configured solve language, falling back to Python.
    pub fn lang(&self) -> Language {
        Language::parse(&self.language).unwrap_or(Language::Python)
    }

    /// The editor command in effect and where it came from.
    pub fn editor(&self) -> (String, &'static str) {
        if let Some(e) = self.editor.as_deref().filter(|e| !e.trim().is_empty()) {
            return (e.to_string(), "config");
        }
        for var in ["VISUAL", "EDITOR"] {
            if let Ok(e) = std::env::var(var)
                && !e.trim().is_empty()
            {
                return (
                    e,
                    if var == "VISUAL" {
                        "$VISUAL"
                    } else {
                        "$EDITOR"
                    },
                );
            }
        }
        ("vi".into(), "default")
    }

    pub fn workspace(&self, paths: &Paths) -> PathBuf {
        self.workspace
            .clone()
            .unwrap_or_else(|| paths.default_workspace.clone())
    }
}

/// Maps friendly editor names to launch commands.
pub fn editor_preset(name: &str) -> Option<&'static str> {
    Some(match name {
        "vscode" | "code" => "code {file}",
        "cursor" => "cursor {file}",
        "zed" => "zed {file}",
        "sublime" | "subl" => "subl {file}",
        "vim" => "vim {file}",
        "nvim" | "neovim" => "nvim {file}",
        "emacs" => "emacs -nw {file}",
        "emacsclient" => "emacsclient -t {file}",
        "helix" | "hx" => "hx {file}",
        "nano" => "nano {file}",
        "micro" => "micro {file}",
        _ => return None,
    })
}

pub const EDITOR_PRESETS: &[&str] = &[
    "vscode",
    "cursor",
    "zed",
    "sublime",
    "vim",
    "nvim",
    "emacs",
    "emacsclient",
    "helix",
    "nano",
    "micro",
];
