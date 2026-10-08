//! The drafting loop and draft workspaces on disk. No UI here.
//!
//! A workspace is `~/.local/state/dojo/contrib/<id>/` holding `state.json`
//! (provider, model, the append-only conversation, the latest draft, the PR
//! once submitted) and the built question under `questions/`.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::draft::{self, Built, Draft};
use super::llm::{Client, Usage};
use crate::questions::Bank;

/// Model rounds per request: the first draft plus fixes for problems dojo
/// finds.
pub const MAX_ROUNDS: u32 = 3;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct State {
    pub provider: String,
    pub model: String,
    /// The conversation, only ever appended to.
    pub messages: Vec<Value>,
    pub draft: Option<Draft>,
    pub pull_request: Option<String>,
    pub input_tokens: u64,
    pub output_tokens: u64,
}

pub struct Workspace {
    pub root: PathBuf,
}

impl Workspace {
    pub fn create(base: &Path) -> Result<Workspace> {
        let id = jiff::Timestamp::now().strftime("%Y%m%d-%H%M%S").to_string();
        let root = base.join(id);
        std::fs::create_dir_all(&root).with_context(|| format!("creating {}", root.display()))?;
        Ok(Workspace { root })
    }

    /// The most recent workspace that hasn't been submitted.
    pub fn latest_open(base: &Path) -> Option<Workspace> {
        let mut dirs: Vec<PathBuf> = std::fs::read_dir(base)
            .ok()?
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.join("state.json").is_file())
            .collect();
        dirs.sort();
        dirs.into_iter()
            .rev()
            .map(|root| Workspace { root })
            .find(|w| {
                w.load()
                    .is_ok_and(|s| s.pull_request.is_none() && s.draft.is_some())
            })
    }

    pub fn load(&self) -> Result<State> {
        let raw = std::fs::read_to_string(self.root.join("state.json"))?;
        Ok(serde_json::from_str(&raw)?)
    }

    pub fn save(&self, state: &State) -> Result<()> {
        let tmp = self.root.join("state.json.tmp");
        std::fs::write(&tmp, serde_json::to_vec_pretty(state)?)?;
        std::fs::rename(&tmp, self.root.join("state.json"))?;
        Ok(())
    }
}

/// The local id a draft is built with: one past the bank's highest. The PR
/// renumbers against the repo at submit time.
pub fn draft_id(bank: &Bank) -> u32 {
    bank.all().iter().map(|q| q.meta.id).max().unwrap_or(0) + 1
}

/// What a request produced.
#[derive(bon::Builder)]
pub struct Outcome {
    pub messages: Vec<Value>,
    pub draft: Draft,
    pub built: Built,
    pub usage: Usage,
    pub rounds: u32,
}

/// Asks the model for the question, builds and validates it, and sends
/// problems back for fixing, up to `MAX_ROUNDS` times. `messages` must end
/// with the user's request. The returned conversation includes every turn.
#[bon::builder]
pub fn draft(
    client: &Client,
    mut messages: Vec<Value>,
    root: &Path,
    progress: &dyn Fn(String),
) -> Result<Outcome> {
    let bank = Bank::embedded();
    let system = draft::system_prompt(&bank);
    let schema = draft::schema();
    let id = draft_id(&bank);
    let mut usage = Usage::default();

    for round in 1..=MAX_ROUNDS {
        progress(if round == 1 {
            format!("drafting with {}", client.model)
        } else {
            format!("fixing problems (round {round}/{MAX_ROUNDS})")
        });
        let turn = client.complete(&system, &messages, &schema)?;
        messages.push(turn.assistant);
        usage += turn.usage;

        let parsed: Draft = match serde_json::from_str(&turn.json) {
            Ok(d) => d,
            Err(e) if round < MAX_ROUNDS => {
                messages.push(Client::user(&format!(
                    "Your output didn't match the schema ({e}). Return the complete question again."
                )));
                continue;
            }
            Err(e) => return Err(e).context("the model's output didn't match the schema"),
        };

        progress("running the solutions and validating".into());
        let built = draft::build()
            .draft(&parsed)
            .root(root)
            .id(id)
            .bank(&bank)
            .call()?;
        if built.problems.is_empty() || round == MAX_ROUNDS {
            return Ok(Outcome::builder()
                .messages(messages)
                .draft(parsed)
                .built(built)
                .usage(usage)
                .rounds(round)
                .build());
        }
        messages.push(Client::user(&format!(
            "dojo built and validated your question and found problems:\n{}\n\nFix them (keeping all six reference solutions consistent) and return the complete corrected question.",
            built
                .problems
                .iter()
                .map(|p| format!("- {p}"))
                .collect::<Vec<_>>()
                .join("\n")
        )));
    }
    unreachable!("the last round always returns")
}

/// Rebuilds a saved draft without the model (resuming a workspace).
pub fn rebuild(draft: &Draft, root: &Path) -> Result<Built> {
    let bank = Bank::embedded();
    draft::build()
        .draft(draft)
        .root(root)
        .id(draft_id(&bank))
        .bank(&bank)
        .call()
}

/// The first message: the contributor's description.
pub fn describe_message(text: &str) -> Value {
    Client::user(&format!(
        "Draft a new question for dojo from this description:\n\n{text}"
    ))
}

/// A follow-up: the contributor's requested changes. Outstanding problems
/// (failing or missing languages included) are sent along to be fixed.
pub fn revise_message(text: &str, problems: &[String]) -> Value {
    let mut msg = format!("The contributor asks for changes:\n\n{text}");
    if !problems.is_empty() {
        msg.push_str("\n\nThe current draft also still has these problems to fix:\n");
        for p in problems {
            msg.push_str(&format!("- {p}\n"));
        }
    }
    msg.push_str(
        "\nApply every change to all six reference solutions (Python, JavaScript, TypeScript, Java, C++, Go) so they stay consistent with each other and the signature, and return the complete updated question.",
    );
    Client::user(&msg)
}

/// The PR description.
pub fn pr_body(draft: &Draft, built: &Built, model: &str) -> String {
    format!(
        "Adds **{}** ({}): {}\n\n- Tags: {}\n- Companies: {}\n- Tests: {} visible, {} hidden ({} generated); expected outputs computed by running the Python reference solution\n- Reference solutions in {} pass every test; starter code is generated from the signature (`dojo scaffold`), loads and doesn't pass\n\nDrafted with `/contribute` ({model}) and validated locally with dojo's question checks.\n",
        draft.title,
        draft.difficulty,
        draft
            .statement
            .lines()
            .find(|l| !l.trim().is_empty())
            .unwrap_or_default()
            .trim(),
        draft.tags.join(", "),
        if draft.companies.is_empty() {
            "—".into()
        } else {
            draft.companies.join(", ")
        },
        built.visible,
        built.hidden,
        built.generated,
        built
            .languages
            .iter()
            .map(|(l, _)| l.label())
            .collect::<Vec<_>>()
            .join(", "),
    )
}
