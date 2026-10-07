mod app;
mod config;
mod lang;
mod model;
mod questions;
mod runner;
mod scaffold;
mod schema;
mod session;
mod store;
mod ui;
mod validate;

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
        Err(e) => {
            eprintln!("error: {e:#}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<ExitCode> {
    let cli = Cli::parse();
    match cli.command {
        Some(Cmd::Validate { dir, no_run }) => {
            let ok = validate::run(&dir, !no_run)?;
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
            for file in schema::write(&out)? {
                println!("wrote {file}");
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
            for dir in dirs {
                for file in scaffold::run(&dir, &langs, force)? {
                    println!("wrote {}/{file}", dir.display());
                }
            }
            Ok(ExitCode::SUCCESS)
        }
        Some(Cmd::Solve { args }) => {
            app::run(Some(format!("/solve {}", args.join(" "))))?;
            Ok(ExitCode::SUCCESS)
        }
        Some(Cmd::Report { json: true, .. }) => {
            println!("{}", app::report_json()?);
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
