//! Compiled languages (Go, Java, C++). dojo generates a driver from the
//! question's signature: it reads the spec, decodes each case's inputs into
//! typed values, calls the solution and writes results with the same
//! protocol as the interpreted harnesses (results file rewritten after each
//! case, progress file naming the current step). The driver and the solution
//! are compiled together in a temporary directory, then run.
//!
//! Per-case time limits come from dojo's watchdog (a step that stops making
//! progress is killed), so drivers don't implement timeouts themselves.

use std::path::Path;
use std::process::Command;
use std::sync::atomic::AtomicBool;
use std::time::Duration;

use anyhow::Result;

use super::exec;
use crate::lang::Language;
use crate::questions::Signature;

pub mod go;

/// Longest a compile may take (the first Go build also compiles the parts
/// of its standard library the driver uses).
const COMPILE_LIMIT: Duration = Duration::from_secs(180);

/// Most compiler output shown for a failed build.
const MAX_ERROR_LINES: usize = 30;

/// What a compiled language needs to build and run one solution.
pub struct Plan {
    /// Files to write into the build directory: (name, contents).
    pub sources: Vec<(String, String)>,
    /// The file name the solution is written as, for error messages.
    pub solution_file: String,
    /// The generated driver's file name, for error messages.
    pub driver_file: String,
    /// Compiles inside the build directory.
    pub compile: Command,
    /// Runs the program; dojo appends the spec and results paths.
    pub run: Command,
}

pub enum Built {
    Ready(Command),
    /// Didn't compile: the message for the user.
    Failed(String),
}

/// Writes the driver and solution into `dir` and compiles them.
#[bon::builder]
pub fn build(
    lang: Language,
    signature: &Signature,
    solution: &Path,
    dir: &Path,
    cancel: &AtomicBool,
) -> Result<Built> {
    let source = std::fs::read_to_string(solution)?;
    let mut plan = match lang {
        Language::Go => go::plan(signature, &source, dir),
        _ => anyhow::bail!("{} isn't a compiled language", lang.label()),
    };
    for (name, body) in &plan.sources {
        std::fs::write(dir.join(name), body)?;
    }
    plan.compile.current_dir(dir);
    let out = exec()
        .cmd(&mut plan.compile)
        .limit(COMPILE_LIMIT)
        .cancel(cancel)
        .call()?;
    if out.timed_out {
        return Ok(Built::Failed(format!(
            "compiling took over {}s and was stopped",
            COMPILE_LIMIT.as_secs()
        )));
    }
    if !out.success {
        let text = format!("{}{}", out.stdout, out.stderr);
        return Ok(Built::Failed(
            compile_error()
                .output(&text)
                .dir(dir)
                .plan(&plan)
                .function(&lang.function_name(&signature.function))
                .call(),
        ));
    }
    plan.run.current_dir(dir);
    Ok(Built::Ready(plan.run))
}

/// The compiler's complaints, shown against the user's file name. Errors in
/// the generated driver mean the solution doesn't match the signature dojo
/// calls, so they're explained as that.
#[bon::builder]
fn compile_error(output: &str, dir: &Path, plan: &Plan, function: &str) -> String {
    let dir = dir.display().to_string();
    let mut driver_errors = false;
    let mut lines = Vec::new();
    for line in output.lines() {
        let line = line
            .replace(&format!("{dir}/"), "")
            .replace(&dir, "")
            .replace("./", "");
        if line.trim().is_empty() || line.starts_with("# ") {
            continue; // Go's "# command-line-arguments" header
        }
        if line.contains(&plan.driver_file) {
            driver_errors = true;
            continue;
        }
        lines.push(line);
    }
    let mut msg = String::from("compile error\n");
    if lines.len() > MAX_ERROR_LINES {
        let more = lines.len() - MAX_ERROR_LINES;
        lines.truncate(MAX_ERROR_LINES);
        lines.push(format!("… ({more} more lines)"));
    }
    msg.push_str(&lines.join("\n"));
    if driver_errors {
        if !lines.is_empty() {
            msg.push('\n');
        }
        msg.push_str(&format!(
            "dojo couldn't call `{function}` from {}: check its name, parameters and return type match the starter code",
            plan.solution_file
        ));
    }
    msg.trim_end().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn plan() -> Plan {
        Plan {
            sources: vec![],
            solution_file: "solution.go".into(),
            driver_file: "dojo_main.go".into(),
            compile: Command::new("true"),
            run: Command::new("true"),
        }
    }

    #[test]
    fn compile_errors_name_the_solution_and_explain_driver_errors() {
        let out = "# command-line-arguments\n\
                   /tmp/x/solution.go:5:2: undefined: foo\n\
                   ./dojo_main.go:20:9: undefined: twoSum\n";
        let msg = compile_error()
            .output(out)
            .dir(Path::new("/tmp/x"))
            .plan(&plan())
            .function("twoSum")
            .call();
        assert_eq!(
            msg,
            "compile error\nsolution.go:5:2: undefined: foo\n\
             dojo couldn't call `twoSum` from solution.go: check its name, parameters and return type match the starter code"
        );
    }

    #[test]
    fn long_compile_output_is_cut() {
        let out: String = (0..50).map(|i| format!("solution.go:{i}: bad\n")).collect();
        let msg = compile_error()
            .output(&out)
            .dir(Path::new("/tmp/x"))
            .plan(&plan())
            .function("f")
            .call();
        assert_eq!(msg.lines().count(), 1 + MAX_ERROR_LINES + 1);
        assert!(msg.ends_with("… (20 more lines)"));
    }
}
