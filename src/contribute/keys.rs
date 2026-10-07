//! API keys for `/contribute`, kept in the OS keychain (macOS Keychain,
//! Secret Service on Linux), never in the config file. The usual environment
//! variables take precedence.

use anyhow::{Context, Result};

use super::llm::Provider;

const SERVICE: &str = "dojo";

fn entry(p: Provider) -> Result<keyring::Entry> {
    keyring::Entry::new(SERVICE, p.name()).context(
        "the OS keychain isn't available here; set the API key as an environment variable instead",
    )
}

/// The key to use, and where it came from.
pub fn get(p: Provider) -> Option<(String, &'static str)> {
    if let Ok(k) = std::env::var(p.env_var())
        && !k.trim().is_empty()
    {
        return Some((k, p.env_var()));
    }
    let key = entry(p).ok()?.get_password().ok()?;
    (!key.trim().is_empty()).then_some((key, "keychain"))
}

pub fn set(p: Provider, key: &str) -> Result<()> {
    entry(p)?
        .set_password(key.trim())
        .context("could not save the key to the OS keychain")
}
