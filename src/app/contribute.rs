//! `/contribute` in the app: describe → draft → review → accept (PR) or
//! revise, plus API-key entry and installing / signing in to `gh`.

use std::time::Instant;

use super::views;
use super::{AfterCommand, App, Entry, Msg};
use crate::contribute::draft::{Built, Draft};
use crate::contribute::llm::{Client, Provider, Usage};
use crate::contribute::workflow::{self, State, Workspace};
use crate::contribute::{github, keys};

pub enum Stage {
    /// Waiting for the description.
    Describe,
    /// A model request (or a rebuild) in flight.
    Working {
        since: Instant,
        status: String,
    },
    /// A built draft is on screen: accept or revise.
    Review,
    /// Asking before running a setup command (install gh or Ollama, update
    /// Ollama, download a model).
    Confirm {
        description: String,
        command: String,
        after: AfterCommand,
    },
    /// Opening the PR.
    Submitting {
        since: Instant,
        status: String,
    },
    Submitted {
        url: String,
    },
}

pub struct Contrib {
    pub ws: Workspace,
    pub state: State,
    pub provider: Provider,
    pub built: Option<Built>,
    pub stage: Stage,
    /// Results from a job other than this one (cancelled) are ignored.
    job: u64,
}

/// Messages from contribute background jobs.
pub enum ContribMsg {
    Progress {
        job: u64,
        status: String,
    },
    Drafted {
        job: u64,
        result: Result<Box<workflow::Outcome>, String>,
    },
    Rebuilt {
        job: u64,
        result: Result<Built, String>,
    },
    Submitted {
        job: u64,
        result: Result<String, String>,
    },
}

fn contrib_base(app: &App) -> std::path::PathBuf {
    app.paths
        .default_workspace
        .parent()
        .map(|state| state.join("contrib"))
        .unwrap_or_else(|| app.paths.default_workspace.join("contrib"))
}

impl App {
    fn provider(&self) -> Provider {
        Provider::parse(&self.config.contribute.provider).unwrap_or(Provider::Claude)
    }

    fn model_for(&self, p: Provider) -> String {
        match p {
            Provider::Claude => self.config.contribute.claude_model.clone(),
            Provider::OpenAi => self.config.contribute.openai_model.clone(),
            Provider::Ollama => self.config.contribute.ollama_model.clone(),
        }
    }

    /// Checks the local model is ready: Ollama installed, its server up,
    /// the model downloaded. Offers to fix the first thing missing (a y/n
    /// question) and returns what to show; `None` means ready.
    fn ollama_gate(&mut self) -> Option<Entry> {
        use crate::contribute::ollama;
        let base = ollama::base_url(&self.config.contribute.ollama_url);
        let model = self.config.contribute.ollama_model.clone();
        let logs = contrib_base(self);
        let c = self.contrib.as_mut()?;
        let mut confirm = |description: String, command: String, after: AfterCommand| {
            c.stage = Stage::Confirm {
                description,
                command,
                after,
            };
        };

        if !ollama::installed() {
            return Some(match ollama::install_plan() {
                Some((description, command)) => {
                    confirm(description.clone(), command, AfterCommand::OllamaInstall);
                    views::warn(format!(
                        "Drafting locally needs Ollama, which isn't installed.\nInstall it now {description}? (y/n)"
                    ))
                }
                None => views::error(
                    "Drafting locally needs Ollama: install it from https://ollama.com/download, then /contribute",
                ),
            });
        }
        if ollama::models(&base).is_none() && !ollama::start_server(&base, &logs) {
            return Some(views::error(format!(
                "couldn't start Ollama at {base}  ·  run `ollama serve` in another terminal, then try again"
            )));
        }
        if !ollama::has_model(&base, &model) {
            confirm(
                format!("download {model}"),
                ollama::pull_command(&model),
                AfterCommand::OllamaPull,
            );
            return Some(views::warn(format!(
                "The model {model} isn't downloaded yet (a one-time download of several GB).\nDownload it now? (y/n)"
            )));
        }
        None
    }

    pub(super) fn cmd_contribute(&mut self, args: &[&str]) -> Option<Entry> {
        match args.first().copied() {
            Some("claude") | Some("openai") | Some("ollama") => {
                let p = Provider::parse(args[0]).unwrap_or(Provider::Claude);
                self.config.contribute.provider = p.name().into();
                if let Err(e) = self.config.save(&self.paths.config_file) {
                    return Some(views::error(format!("could not save config: {e:#}")));
                }
                self.transcript.push(views::ok(format!(
                    "contributions now drafted with {} ({})",
                    p.label(),
                    self.model_for(p)
                )));
                if let Some(c) = self.contrib.as_mut() {
                    c.provider = p;
                }
                if p.needs_key() && keys::get(p).is_none() {
                    return self.ask_key(p);
                }
                None
            }
            Some("key") => self.ask_key(self.provider()),
            Some("new") => self.start_contrib(true),
            Some(other) => Some(views::error(format!(
                "unknown option `{other}`  ·  /contribute [new | key | claude | openai | ollama]"
            ))),
            None => self.start_contrib(false),
        }
    }

    fn ask_key(&mut self, p: Provider) -> Option<Entry> {
        self.secret_for = Some(p);
        self.input.set("");
        Some(views::info(format!(
            "Paste your {} API key and press Enter. It's stored in your OS keychain, never in a file,\nand only sent to {} when drafting. ({} also works.)",
            p.label(),
            p.label(),
            p.env_var()
        )))
    }

    /// The secret was entered (input masked, not echoed or kept in history).
    pub(super) fn on_secret(&mut self, key: String) {
        let Some(p) = self.secret_for.take() else {
            return;
        };
        if key.trim().is_empty() {
            self.transcript.push(views::info("no key entered"));
            return;
        }
        match keys::set(p, &key) {
            Ok(()) => {
                self.transcript.push(views::ok(format!(
                    "{} key saved to your keychain",
                    p.label()
                )));
                if self.contrib.is_none()
                    && let Some(e) = self.start_contrib(false)
                {
                    self.transcript.push(e);
                }
            }
            Err(e) => self.transcript.push(views::error(format!(
                "{e:#}  ·  or export {} before starting dojo",
                p.env_var()
            ))),
        }
    }

    /// Starts a new draft, or resumes the latest unsubmitted one.
    fn start_contrib(&mut self, fresh: bool) -> Option<Entry> {
        let p = self.provider();
        if p.needs_key() && keys::get(p).is_none() {
            return self.ask_key(p);
        }
        let base = contrib_base(self);
        let resumed = if fresh {
            None
        } else {
            Workspace::latest_open(&base)
        };
        match resumed {
            Some(ws) => {
                let state = match ws.load() {
                    Ok(s) => s,
                    Err(e) => {
                        return Some(views::error(format!("could not load the draft: {e:#}")));
                    }
                };
                let draft = state.draft.clone()?;
                let provider = Provider::parse(&state.provider).unwrap_or(p);
                self.job_seq += 1;
                let job = self.job_seq;
                let root = ws.root.clone();
                let tx = self.tx.clone();
                std::thread::spawn(move || {
                    let result = workflow::rebuild(&draft, &root).map_err(|e| format!("{e:#}"));
                    let _ = tx.send(Msg::Contrib(ContribMsg::Rebuilt { job, result }));
                });
                self.contrib = Some(Contrib {
                    ws,
                    state,
                    provider,
                    built: None,
                    stage: Stage::Working {
                        since: Instant::now(),
                        status: "re-checking your draft".into(),
                    },
                    job,
                });
                Some(views::info(
                    "continuing your unfinished draft  ·  /contribute new starts another",
                ))
            }
            None => {
                let ws = match Workspace::create(&base) {
                    Ok(ws) => ws,
                    Err(e) => return Some(views::error(format!("{e:#}"))),
                };
                let model = self.model_for(p);
                self.contrib = Some(Contrib {
                    ws,
                    state: State {
                        provider: p.name().into(),
                        model: model.clone(),
                        ..State::default()
                    },
                    provider: p,
                    built: None,
                    stage: Stage::Describe,
                    job: 0,
                });
                let intro = views::contrib_intro(p, &model);
                if p == Provider::Ollama
                    && let Some(setup) = self.ollama_gate()
                {
                    self.transcript.push(intro);
                    return Some(setup);
                }
                Some(intro)
            }
        }
    }

    /// Plain text while contributing: the description, a requested change,
    /// or an answer to a yes/no question. Returns `false` when not consumed.
    pub(super) fn contrib_text(&mut self, text: &str) -> bool {
        let Some(c) = self.contrib.as_mut() else {
            return false;
        };
        match &c.stage {
            Stage::Describe | Stage::Review => {}
            Stage::Confirm { command, after, .. } => {
                let (command, after) = (command.clone(), *after);
                let yes = matches!(text.to_ascii_lowercase().as_str(), "y" | "yes");
                c.stage = if c.state.draft.is_some() {
                    Stage::Review
                } else {
                    Stage::Describe
                };
                if yes {
                    self.pending_command = Some((with_pause(&command), after));
                } else {
                    self.transcript
                        .push(views::info("ok, skipped  ·  run /contribute again when you're ready"));
                }
                return true;
            }
            Stage::Working { .. } | Stage::Submitting { .. } => {
                self.notify("still working  ·  Ctrl+C cancels");
                return true;
            }
            Stage::Submitted { .. } => return false,
        }

        let provider = c.provider;
        let key = if !provider.needs_key() {
            String::new()
        } else {
            match keys::get(provider) {
                Some((k, _)) => k,
                None => {
                    if let Some(e) = self.ask_key(provider) {
                        self.transcript.push(e);
                    }
                    return true;
                }
            }
        };
        // A local model must be installed, running and downloaded first.
        if provider == Provider::Ollama
            && let Some(e) = self.ollama_gate()
        {
            self.transcript.push(e);
            return true;
        }
        let base_url = crate::contribute::ollama::base_url(&self.config.contribute.ollama_url);
        let Some(c) = self.contrib.as_mut() else {
            return true;
        };
        let mut messages = c.state.messages.clone();
        messages.push(match c.state.draft {
            None => workflow::describe_message(text),
            Some(_) => workflow::revise_message(
                text,
                c.built
                    .as_ref()
                    .map(|b| b.problems.as_slice())
                    .unwrap_or(&[]),
            ),
        });
        let client = match Client::new(c.provider, c.state.model.clone(), key, base_url) {
            Ok(cl) => cl,
            Err(e) => {
                self.transcript.push(views::error(format!("{e:#}")));
                return true;
            }
        };
        self.job_seq += 1;
        let job = self.job_seq;
        c.job = job;
        c.stage = Stage::Working {
            since: Instant::now(),
            status: format!("drafting with {}", c.state.model),
        };
        let root = c.ws.root.clone();
        let tx = self.tx.clone();
        std::thread::spawn(move || {
            let progress_tx = tx.clone();
            let progress = move |status: String| {
                let _ = progress_tx.send(Msg::Contrib(ContribMsg::Progress { job, status }));
            };
            let result = workflow::draft(&client, messages, &root, &progress)
                .map(Box::new)
                .map_err(|e| format!("{e:#}"));
            let _ = tx.send(Msg::Contrib(ContribMsg::Drafted { job, result }));
        });
        true
    }

    pub(super) fn on_contrib(&mut self, msg: ContribMsg) {
        let Some(c) = self.contrib.as_mut() else {
            return;
        };
        let current = |job: u64| job == c.job;
        match msg {
            ContribMsg::Progress { job, status } if current(job) => match &mut c.stage {
                Stage::Working { status: s, .. } | Stage::Submitting { status: s, .. } => {
                    *s = status
                }
                _ => {}
            },
            ContribMsg::Drafted { job, result } if current(job) => match result {
                Ok(out) => {
                    c.state.messages = out.messages;
                    c.state.draft = Some(out.draft.clone());
                    c.state.input_tokens += out.usage.input_tokens;
                    c.state.output_tokens += out.usage.output_tokens;
                    if let Err(e) = c.ws.save(&c.state) {
                        self.notice =
                            Some((format!("could not save the draft: {e:#}"), Instant::now()));
                    }
                    let entry = views::contrib_review(
                        &out.draft,
                        &out.built,
                        &c.state.model,
                        out.rounds,
                        Usage {
                            input_tokens: c.state.input_tokens,
                            output_tokens: c.state.output_tokens,
                            cache_read_tokens: 0,
                        },
                    );
                    c.built = Some(out.built);
                    c.stage = Stage::Review;
                    self.transcript.push(entry);
                }
                Err(e) => {
                    c.stage = if c.state.draft.is_some() {
                        Stage::Review
                    } else {
                        Stage::Describe
                    };
                    self.transcript.push(views::error(format!(
                        "{e}  ·  send your message again to retry"
                    )));
                }
            },
            ContribMsg::Rebuilt { job, result } if current(job) => match result {
                Ok(built) => {
                    let draft = c.state.draft.clone();
                    c.stage = Stage::Review;
                    if let Some(d) = draft {
                        self.transcript.push(views::contrib_review(
                            &d,
                            &built,
                            &c.state.model,
                            0,
                            Usage {
                                input_tokens: c.state.input_tokens,
                                output_tokens: c.state.output_tokens,
                                cache_read_tokens: 0,
                            },
                        ));
                    }
                    c.built = Some(built);
                }
                Err(e) => {
                    c.stage = Stage::Review;
                    self.transcript
                        .push(views::error(format!("could not rebuild the draft: {e}")));
                }
            },
            ContribMsg::Submitted { job, result } if current(job) => match result {
                Ok(url) => {
                    c.state.pull_request = Some(url.clone());
                    let _ = c.ws.save(&c.state);
                    c.stage = Stage::Submitted { url: url.clone() };
                    self.transcript.push(views::contrib_submitted(&url));
                }
                Err(e) => {
                    c.stage = Stage::Review;
                    self.transcript.push(views::error(format!(
                        "{e}  ·  /accept to try again; the draft is in {}",
                        c.ws.root.display()
                    )));
                }
            },
            _ => {} // a cancelled job
        }
    }

    /// Ctrl+C while drafting or submitting: stop waiting for the result.
    pub(super) fn cancel_contrib(&mut self) -> bool {
        let Some(c) = self.contrib.as_mut() else {
            return false;
        };
        if !matches!(c.stage, Stage::Working { .. } | Stage::Submitting { .. }) {
            return false;
        }
        c.job = u64::MAX; // ignore whatever comes back
        c.stage = if c.state.draft.is_some() {
            Stage::Review
        } else {
            Stage::Describe
        };
        self.transcript.push(views::info("cancelled"));
        true
    }

    /// `/accept`: open the PR, installing and signing in to `gh` first if
    /// needed.
    pub(super) fn cmd_accept(&mut self) -> Option<Entry> {
        let Some(c) = self.contrib.as_mut() else {
            return Some(views::error(
                "nothing to accept  ·  /contribute to draft a question",
            ));
        };
        let (Some(draft), Some(built)) = (c.state.draft.clone(), c.built.clone()) else {
            return Some(views::error("no draft yet  ·  describe the question first"));
        };
        if !matches!(c.stage, Stage::Review) {
            return Some(views::info("not ready yet"));
        }
        if !built.problems.is_empty() {
            return Some(views::error(
                "the draft still has problems  ·  describe a fix (or ask to fix them) first",
            ));
        }
        // The folder may have been edited by hand since it was built:
        // re-check it as it is on disk before anything leaves this machine.
        let recheck = match (built.dir.parent(), built.dir.file_name()) {
            (Some(root), Some(name)) => {
                crate::validate::check_question(root, &name.to_string_lossy(), true)
            }
            _ => vec!["the draft folder is missing".into()],
        };
        if !recheck.is_empty() {
            if let Some(b) = c.built.as_mut() {
                b.problems = recheck.clone();
            }
            return Some(views::error(format!(
                "the draft no longer passes validation, so no PR was opened:\n{}",
                recheck
                    .iter()
                    .map(|p| format!("- {p}"))
                    .collect::<Vec<_>>()
                    .join("\n")
            )));
        }
        let Some(gh) = github::gh() else {
            let (description, command) = github::install_plan();
            c.stage = Stage::Confirm {
                description: format!("install gh {description}"),
                command,
                after: AfterCommand::GhInstall,
            };
            return Some(views::warn(format!(
                "Opening the PR needs the GitHub CLI (gh), which isn't installed.\nInstall it now {description}? (y/n)"
            )));
        };
        if !github::signed_in(&gh) {
            self.pending_command = Some((with_pause(&github::login_command(&gh)), AfterCommand::GhLogin));
            return Some(views::info("signing you in to GitHub with gh…"));
        }

        self.job_seq += 1;
        let job = self.job_seq;
        c.job = job;
        c.stage = Stage::Submitting {
            since: Instant::now(),
            status: "starting".into(),
        };
        let model = c.state.model.clone();
        let tx = self.tx.clone();
        std::thread::spawn(move || {
            let progress_tx = tx.clone();
            let progress = move |s: &str| {
                let _ = progress_tx.send(Msg::Contrib(ContribMsg::Progress {
                    job,
                    status: s.to_string(),
                }));
            };
            let sub = github::Submission {
                question_dir: &built.dir,
                slug: &draft.slug,
                title: &draft.title,
                body: workflow::pr_body(&draft, &built, &model),
            };
            let result = github::submit(&gh, &sub, &progress).map_err(|e| format!("{e:#}"));
            let _ = tx.send(Msg::Contrib(ContribMsg::Submitted { job, result }));
        });
        None
    }

    /// After the `gh` install or sign-in ran in the terminal: carry on.
    pub(super) fn after_setup(&mut self, after: AfterCommand, ok: bool) {
        match after {
            AfterCommand::OllamaPull
                if !crate::contribute::ollama::has_model(
                    &crate::contribute::ollama::base_url(&self.config.contribute.ollama_url),
                    &self.config.contribute.ollama_model,
                ) =>
            {
                // The usual reason: the model needs a newer Ollama.
                let update = crate::contribute::ollama::update_plan();
                match (update, self.contrib.as_mut()) {
                    (Some((description, command)), Some(c)) => {
                        c.stage = Stage::Confirm {
                            description: description.clone(),
                            command,
                            after: AfterCommand::OllamaUpdate,
                        };
                        self.transcript.push(views::warn(format!(
                            "The download didn't finish. If it said the model needs a newer Ollama,\nupdate Ollama now {description}? (y/n)"
                        )));
                    }
                    _ => self.transcript.push(views::error(
                        "the download didn't finish  ·  update Ollama (https://ollama.com/download), then try again",
                    )),
                }
            }
            AfterCommand::OllamaInstall | AfterCommand::OllamaUpdate | AfterCommand::OllamaPull => {
                match self.ollama_gate() {
                    Some(e) => self.transcript.push(e),
                    None => self.transcript.push(views::ok(format!(
                        "Ollama is ready with {}  ·  describe the question (or your change) to draft",
                        self.config.contribute.ollama_model
                    ))),
                }
            }
            AfterCommand::GhInstall if github::gh().is_none() => {
                self.transcript.push(views::error(if ok {
                    "gh still isn't on your PATH  ·  open a new terminal, then /accept"
                } else {
                    "installing gh failed  ·  see https://cli.github.com, then /accept"
                }));
            }
            AfterCommand::GhLogin if !github::gh().is_some_and(|gh| github::signed_in(&gh)) => {
                self.transcript.push(views::error(
                    "not signed in to GitHub  ·  /accept to try again",
                ));
            }
            _ => {
                if let Some(e) = self.cmd_accept() {
                    self.transcript.push(e);
                }
            }
        }
    }

    /// Whether `/accept` makes sense right now.
    pub(super) fn contrib_reviewing(&self) -> bool {
        self.contrib
            .as_ref()
            .is_some_and(|c| matches!(c.stage, Stage::Review))
    }

    /// The empty prompt's guidance while contributing.
    pub(super) fn contrib_prompt(&self) -> Option<Vec<(String, String)>> {
        let tip = |k: &str, t: &str| (k.to_string(), t.to_string());
        if let Some(p) = self.secret_for {
            return Some(vec![tip(
                "",
                &format!(
                    "paste your {} API key · stored in your OS keychain · Esc cancels",
                    p.label()
                ),
            )]);
        }
        let c = self.contrib.as_ref()?;
        Some(match &c.stage {
            Stage::Describe => vec![tip(
                "",
                "describe the question you'd like to add, then press Enter",
            )],
            Stage::Working { status, since } | Stage::Submitting { status, since } => vec![
                tip("", &format!("{status}… {}s", since.elapsed().as_secs())),
                tip("Ctrl+C", "cancel"),
            ],
            Stage::Review if c.built.as_ref().is_some_and(|b| !b.problems.is_empty()) => vec![
                tip("type", "a change, or \"fix the problems\""),
                tip("/contribute new", "start over"),
            ],
            Stage::Review => vec![
                tip("/accept", "open the pull request"),
                tip("type", "a change to revise"),
                tip("/contribute new", "start another"),
            ],
            Stage::Confirm { description, .. } => {
                vec![tip("y", description), tip("n", "not now")]
            }
            Stage::Submitted { url } => vec![
                tip("", &format!("opened {url}")),
                tip("/contribute", "add another question"),
            ],
        })
    }
}

/// `Draft` access for views.
pub fn signature(d: &Draft) -> String {
    let params: Vec<String> = d
        .params
        .iter()
        .map(|p| format!("{}: {}", p.name, p.ty))
        .collect();
    format!("{}({}) -> {}", d.function, params.join(", "), d.returns)
}

/// Runs `command`, then waits for Enter so its output can be read before
/// dojo takes the terminal back.
fn with_pause(command: &str) -> String {
    format!(
        "{command}; status=$?; echo; read -r -p 'Press Enter to return to dojo... ' _; exit $status"
    )
}
