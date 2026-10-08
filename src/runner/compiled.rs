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

pub mod cpp;
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
        Language::Cpp => cpp::plan(signature, &source, dir),
        Language::Go => go::plan(signature, &source, dir),
        _ => anyhow::bail!("{} isn't a compiled language", lang.label()),
    };
    for (name, body) in &plan.sources {
        let path = dir.join(name);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, body)?;
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
/// dojo's own files (the generated driver, support code) mean the solution
/// doesn't match the signature dojo calls, so they're explained as that
/// instead, along with the notes and source excerpts that follow them.
#[bon::builder]
fn compile_error(output: &str, dir: &Path, plan: &Plan, function: &str) -> String {
    let dir = dir.display().to_string();
    let dojo_files: Vec<&str> = std::iter::once(plan.driver_file.as_str())
        .chain(plan.sources.iter().map(|(name, _)| name.as_str()))
        .filter(|name| *name != plan.solution_file)
        .collect();
    let mut driver_errors = false;
    // Whether the diagnostic being read (and its notes and excerpts) is hidden.
    let mut hidden = false;
    let mut lines = Vec::new();
    for line in output.lines() {
        let line = line
            .replace(&format!("{dir}/"), "")
            .replace(&dir, "")
            .replace("./", "");
        if line.trim().is_empty() || line.starts_with("# ") || generated_count(&line) {
            continue; // Go's "# command-line-arguments", clang's "1 error generated."
        }
        match location(&line) {
            // A note belongs to the diagnostic before it.
            Some((_, rest)) if rest.starts_with("note:") => {}
            // GCC's instantiation trail ("required from here") comes before
            // its error: hidden when it's in dojo's files.
            Some((file, rest)) if rest.starts_with("required from") => {
                hidden = dojo_files.contains(&file);
            }
            Some((file, rest)) => {
                hidden = dojo_files.contains(&file);
                if hidden && !rest.starts_with("warning") {
                    driver_errors = true;
                }
            }
            // Context about dojo's files: "In file included from
            // dojo_main.cpp:3:", GCC's "dojo_main.cpp: In function ...".
            None if dojo_files.iter().any(|f| line.contains(f)) => continue,
            None => {}
        }
        if !hidden {
            lines.push(line);
        }
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

/// `file:line[:col]: rest` → (file, rest).
fn location(line: &str) -> Option<(&str, &str)> {
    let (file, after) = line.split_once(':')?;
    if file.is_empty() || file.contains(char::is_whitespace) {
        return None;
    }
    let digits = after.find(|c: char| !c.is_ascii_digit())?;
    if digits == 0 || !after[digits..].starts_with(':') {
        return None;
    }
    let mut rest = &after[digits + 1..];
    // The column, when there is one.
    if let Some(col) = rest.find(|c: char| !c.is_ascii_digit())
        && col > 0
        && rest[col..].starts_with(':')
    {
        rest = &rest[col + 1..];
    }
    Some((file, rest.trim_start()))
}

/// Clang's closing count, e.g. "2 errors generated."
fn generated_count(line: &str) -> bool {
    let line = line.trim();
    line.starts_with(|c: char| c.is_ascii_digit())
        && (line.ends_with(" generated.") || line.ends_with(" generated"))
        && (line.contains(" error") || line.contains(" warning"))
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

    fn cpp_plan() -> Plan {
        Plan {
            sources: vec![
                ("dojo_main.cpp".into(), String::new()),
                ("dojo_support.hpp".into(), String::new()),
                ("solution.cpp".into(), String::new()),
            ],
            solution_file: "solution.cpp".into(),
            driver_file: "dojo_main.cpp".into(),
            compile: Command::new("true"),
            run: Command::new("true"),
        }
    }

    fn cpp_error(out: &str) -> String {
        compile_error()
            .output(out)
            .dir(Path::new("/tmp/x"))
            .plan(&cpp_plan())
            .function("twoSum")
            .call()
    }

    const CALL_HINT: &str = "dojo couldn't call `twoSum` from solution.cpp: check its name, parameters and return type match the starter code";

    /// Clang: the solution's own errors keep their excerpts and notes; the
    /// driver's and support header's errors are hidden with theirs.
    #[test]
    fn clang_errors_hide_dojo_files() {
        let out = "In file included from dojo_main.cpp:3:\n\
                   ./solution.cpp:1:8: error: redefinition of 'ListNode'\n\
                   \x20   1 | struct ListNode { int v; };\n\
                   \x20     |        ^\n\
                   ./dojo_support.hpp:62:8: note: previous definition is here\n\
                   \x20  62 | struct ListNode {\n\
                   \x20     |        ^\n\
                   ./dojo_support.hpp:529:23: error: static assertion failed: dojo can't return this type\n\
                   \x20 529 |         static_assert(always_false<U>::value, \"dojo can't return this type\");\n\
                   ./dojo_support.hpp:525:20: note: in instantiation of function template specialization requested here\n\
                   dojo_main.cpp:9:18: note: in instantiation of function template specialization requested here\n\
                   \x20   9 |     return dojo::encode(solution.twoSum(a0, a1));\n\
                   ./solution.cpp:3:17: note: 'twoSum' declared here\n\
                   2 errors generated.\n";
        assert_eq!(
            cpp_error(out),
            format!(
                "compile error\n\
                 solution.cpp:1:8: error: redefinition of 'ListNode'\n\
                 \x20   1 | struct ListNode {{ int v; }};\n\
                 \x20     |        ^\n\
                 dojo_support.hpp:62:8: note: previous definition is here\n\
                 \x20  62 | struct ListNode {{\n\
                 \x20     |        ^\n\
                 {CALL_HINT}"
            )
        );
    }

    /// GCC: function context lines and the instantiation trail into dojo's
    /// files are hidden too.
    #[test]
    fn gcc_errors_hide_dojo_files() {
        let out = "In file included from dojo_main.cpp:3:\n\
                   solution.cpp: In member function 'bool Solution::negate(bool)':\n\
                   solution.cpp:4:20: error: expected primary-expression before ';' token\n\
                   \x20   4 |         return flag +;\n\
                   dojo_main.cpp: In function 'std::string dojo_case(const dojo::Json&)':\n\
                   dojo_main.cpp:7:37: error: no matching function for call to 'Solution::negate()'\n\
                   \x20   7 |     return dojo::encode(solution.negate());\n\
                   solution.cpp:3:10: note: candidate: 'bool Solution::negate(bool)'\n";
        assert_eq!(
            cpp_error(out),
            format!(
                "compile error\n\
                 solution.cpp: In member function 'bool Solution::negate(bool)':\n\
                 solution.cpp:4:20: error: expected primary-expression before ';' token\n\
                 \x20   4 |         return flag +;\n\
                 {CALL_HINT}"
            )
        );
        let out = "dojo_support.hpp: In instantiation of 'std::string dojo::encode(const T&)':\n\
                   dojo_support.hpp:525:27:   required from 'std::string dojo::encode(const T&)'\n\
                   dojo_main.cpp:9:24:   required from here\n\
                   dojo_support.hpp:529:23: error: static assertion failed: dojo can't return this type\n";
        assert_eq!(cpp_error(out), format!("compile error\n{CALL_HINT}"));
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
