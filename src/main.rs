mod app;
mod config;
mod contribute;
mod lang;
mod model;
mod project;
mod questions;
mod runner;
mod scaffold;
mod schema;
mod select;
mod session;
mod store;
mod ui;
mod validate;

use std::io::Write;
use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::Result;
use clap::{Parser, Subcommand};

/// Coding-interview practice in your terminal.
#[derive(Parser)]
#[command(name = "dojo", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Cmd>,
}

#[derive(Subcommand)]
enum Cmd {
    /// Check a questions directory: schema, test cases and reference solutions.
    Validate {
        /// Path to the questions directory.
        #[arg(default_value = "questions")]
        dir: PathBuf,
        /// Skip running reference solutions.
        #[arg(long)]
        no_run: bool,
        /// Only check these question folders (e.g. 0018-word-ladder).
        #[arg(long = "question", value_name = "FOLDER")]
        questions: Vec<String>,
    },
    /// Write JSON Schemas for meta.json and tests.json.
    Schema {
        #[arg(long, default_value = "schema")]
        out: PathBuf,
    },
    /// Draft missing boilerplate files from each question's signature.
    Scaffold {
        /// Question directories (e.g. questions/0001-two-sum).
        #[arg(required = true)]
        dirs: Vec<PathBuf>,
        /// Languages to draft (default: all supported).
        #[arg(long, value_delimiter = ',')]
        lang: Vec<String>,
        /// Overwrite existing boilerplate.
        #[arg(long)]
        force: bool,
    },
    /// Open dojo and start a session (`dojo solve 12`, `dojo solve dfs -n 3`).
    Solve {
        #[arg(required = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
    /// Open dojo and draft a new question to contribute.
    Contribute,
    /// Open the report, or print it as JSON with --json.
    Report {
        /// Print the report as JSON instead of opening dojo.
        #[arg(long)]
        json: bool,
        /// Focus on one tag, company or difficulty.
        label: Option<String>,
    },
    /// Open dojo and show a problem.
    Show {
        /// Question id, slug or search words.
        #[arg(required = true)]
        question: Vec<String>,
    },
    /// Open dojo and list questions.
    List {
        /// Tags, companies, difficulty or search words.
        query: Vec<String>,
    },
}

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        // `dojo validate | head`: the reader went away, which isn't an error.
        Err(e) if broken_pipe(&e) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e:#}");
            ExitCode::FAILURE
        }
    }
}

/// Whether writing to stdout failed because the reading end closed. Rust
/// ignores SIGPIPE, so a closed pipe shows up as this error instead of
/// killing dojo (which the clipboard's pipes to xclip/wl-copy rely on).
fn broken_pipe(e: &anyhow::Error) -> bool {
    e.chain().any(|cause| {
        cause
            .downcast_ref::<std::io::Error>()
            .is_some_and(|io| io.kind() == std::io::ErrorKind::BrokenPipe)
    })
}

fn run() -> Result<ExitCode> {
    let cli = Cli::parse();
    match cli.command {
        Some(Cmd::Validate {
            dir,
            no_run,
            questions,
        }) => {
            let ok = validate::run(&dir, &questions, !no_run)?;
            Ok(if ok {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            })
        }
        None => {
            app::run(None)?;
            Ok(ExitCode::SUCCESS)
        }
        Some(Cmd::Schema { out }) => {
            let mut stdout = std::io::stdout().lock();
            for file in schema::write(&out)? {
                writeln!(stdout, "wrote {file}")?;
            }
            Ok(ExitCode::SUCCESS)
        }
        Some(Cmd::Scaffold { dirs, lang, force }) => {
            let langs = if lang.is_empty() {
                lang::Language::ALL.to_vec()
            } else {
                lang.iter()
                    .map(|l| {
                        lang::Language::parse(l)
                            .ok_or_else(|| anyhow::anyhow!("unknown language `{l}`"))
                    })
                    .collect::<Result<Vec<_>>>()?
            };
            let mut stdout = std::io::stdout().lock();
            for dir in dirs {
                for file in scaffold::run(&dir, &langs, force)? {
                    writeln!(stdout, "wrote {}/{file}", dir.display())?;
                }
            }
            Ok(ExitCode::SUCCESS)
        }
        Some(Cmd::Solve { args }) => {
            app::run(Some(format!("/solve {}", args.join(" "))))?;
            Ok(ExitCode::SUCCESS)
        }
        Some(Cmd::Contribute) => {
            app::run(Some("/contribute".into()))?;
            Ok(ExitCode::SUCCESS)
        }
        Some(Cmd::Report { json: true, .. }) => {
            writeln!(std::io::stdout().lock(), "{}", app::report_json()?)?;
            Ok(ExitCode::SUCCESS)
        }
        Some(Cmd::Report { json: false, label }) => {
            let line = match label {
                Some(l) => format!("/report {l}"),
                None => "/report".into(),
            };
            app::run(Some(line))?;
            Ok(ExitCode::SUCCESS)
        }
        Some(Cmd::Show { question }) => {
            app::run(Some(format!("/show {}", question.join(" "))))?;
            Ok(ExitCode::SUCCESS)
        }
        Some(Cmd::List { query }) => {
            app::run(Some(
                format!("/list {}", query.join(" ")).trim_end().to_string(),
            ))?;
            Ok(ExitCode::SUCCESS)
        }
    }
}
