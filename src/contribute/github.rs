//! Opening a contribution PR with the GitHub CLI (`gh`): installing and
//! signing in to it when needed, then fork → branch → commit → push → PR.

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use anyhow::{Context, Result, bail};

use crate::project::REPO;

fn on_path(program: &str) -> bool {
    std::env::var_os("PATH")
        .is_some_and(|path| std::env::split_paths(&path).any(|dir| dir.join(program).is_file()))
}

/// `gh`, preferring PATH and falling back to `~/.local/bin`, where dojo
/// installs it when no package manager is available.
pub fn gh() -> Option<PathBuf> {
    if on_path("gh") {
        return Some(PathBuf::from("gh"));
    }
    let local = directories::BaseDirs::new()?
        .home_dir()
        .join(".local/bin/gh");
    local.is_file().then_some(local)
}

/// How to install `gh` on this machine: a description and a shell command
/// to run with the terminal handed over (package managers may ask for a
/// sudo password).
pub fn install_plan() -> (String, String) {
    let candidates: &[(&str, &str, &str)] = &[
        ("brew", "Homebrew", "brew install gh"),
        (
            "apt-get",
            "apt",
            "sudo apt-get update && sudo apt-get install -y gh",
        ),
        ("dnf", "dnf", "sudo dnf install -y gh"),
        ("pacman", "pacman", "sudo pacman -S --noconfirm github-cli"),
        ("zypper", "zypper", "sudo zypper install -y gh"),
    ];
    for (program, name, command) in candidates {
        if on_path(program) {
            return (format!("with {name}: {command}"), command.to_string());
        }
    }
    (
        "from GitHub's releases into ~/.local/bin (no admin rights needed)".into(),
        release_install_script(),
    )
}

/// Downloads the latest official `gh` release into `~/.local/bin`.
fn release_install_script() -> String {
    let os = if cfg!(target_os = "macos") {
        "macOS"
    } else {
        "linux"
    };
    let arch = if cfg!(target_arch = "aarch64") {
        "arm64"
    } else {
        "amd64"
    };
    let ext = if cfg!(target_os = "macos") {
        "zip"
    } else {
        "tar.gz"
    };
    format!(
        r#"set -e
tag=$(curl -fsSL https://api.github.com/repos/cli/cli/releases/latest | sed -n 's/.*"tag_name": *"v\([^"]*\)".*/\1/p' | head -1)
name="gh_${{tag}}_{os}_{arch}"
tmp=$(mktemp -d)
echo "Downloading gh ${{tag}}..."
curl -fsSL "https://github.com/cli/cli/releases/download/v${{tag}}/${{name}}.{ext}" -o "$tmp/gh.{ext}"
cd "$tmp"
if [ "{ext}" = "zip" ]; then unzip -q gh.zip; else tar -xzf gh.tar.gz; fi
mkdir -p "$HOME/.local/bin"
install -m 755 "$tmp/$name/bin/gh" "$HOME/.local/bin/gh"
echo "Installed gh to ~/.local/bin/gh""#
    )
}

pub fn signed_in(gh: &Path) -> bool {
    Command::new(gh)
        .args(["auth", "status", "--hostname", "github.com"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|s| s.success())
}

pub fn login_command(gh: &Path) -> String {
    format!(
        "'{}' auth login --hostname github.com --git-protocol https --web",
        gh.display()
    )
}

fn run(cmd: &mut Command, what: &str) -> Result<String> {
    let out = cmd
        .stdin(Stdio::null())
        .output()
        .with_context(|| format!("could not run {what}"))?;
    if !out.status.success() {
        bail!(
            "{what} failed: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

/// What the PR contains and says.
#[derive(bon::Builder)]
pub struct Submission<'a> {
    /// The validated question folder (`…/questions/NNNN-slug`).
    pub question_dir: &'a Path,
    pub slug: &'a str,
    pub title: &'a str,
    pub body: String,
}

/// Forks the repo (or uses it directly when you own it), adds the question
/// on a new branch with the next free id, pushes, and opens the PR.
/// Returns the PR URL. `progress` reports each step.
pub fn submit(gh: &Path, sub: &Submission, progress: &dyn Fn(&str)) -> Result<String> {
    let (owner, name) = REPO.split_once('/').context("bad repo name")?;
    progress("checking your GitHub account…");
    let login = run(
        Command::new(gh).args(["api", "user", "--jq", ".login"]),
        "gh api user",
    )?;

    let work = tempfile::tempdir()?;
    let clone = work.path().join(name);
    let own = login.eq_ignore_ascii_case(owner);
    if own {
        progress(&format!("cloning {REPO}…"));
        run(
            Command::new(gh)
                .args(["repo", "clone", REPO])
                .arg(&clone)
                .args(["--", "--depth", "50"]),
            "gh repo clone",
        )?;
    } else {
        progress(&format!("forking {REPO} to {login}/{name}…"));
        run(
            Command::new(gh).current_dir(work.path()).args([
                "repo",
                "fork",
                REPO,
                "--clone",
                "--default-branch-only",
            ]),
            "gh repo fork",
        )?;
        // Branch from the upstream default branch, not a possibly stale fork.
        run(
            Command::new("git")
                .current_dir(&clone)
                .args(["fetch", "upstream"]),
            "git fetch upstream",
        )?;
    }
    let base_ref = if own { "origin/HEAD" } else { "upstream/HEAD" };
    let base_branch = run(
        Command::new("git")
            .current_dir(&clone)
            .args(["rev-parse", "--abbrev-ref", base_ref]),
        "git rev-parse",
    )
    .unwrap_or_else(|_| format!("{}/main", if own { "origin" } else { "upstream" }));
    let base = base_branch
        .split_once('/')
        .map_or("main".to_string(), |(_, b)| b.to_string());

    let branch = format!("question/{}", sub.slug);
    run(
        Command::new("git")
            .current_dir(&clone)
            .args(["checkout", "-B", &branch, &base_branch]),
        "git checkout",
    )?;

    progress("adding the question…");
    let questions = clone.join("questions");
    // Ids taken by open question PRs count too, so concurrent contributions
    // don't collide.
    let claimed = claimed_ids(gh);
    let id = next_id(&questions)?.max(claimed.iter().max().map_or(0, |m| m + 1));
    let folder = format!("{id:04}-{}", sub.slug);
    let dest = questions.join(&folder);
    copy_question(sub.question_dir, &dest, id)?;

    // Validate exactly what would be committed, against the current bank:
    // nothing is pushed unless it passes here.
    progress("validating the question in the repo…");
    let mut problems = crate::validate::check_question(&questions, &folder, true);
    problems.extend(crate::validate::check_bank(&questions)?);
    if !problems.is_empty() {
        bail!(
            "the question failed validation against the latest repo, so no PR was opened:\n{}",
            problems
                .iter()
                .map(|p| format!("- {p}"))
                .collect::<Vec<_>>()
                .join("\n")
        );
    }

    run(
        Command::new("git")
            .current_dir(&clone)
            .args(["add", "questions"]),
        "git add",
    )?;
    run(
        Command::new("git").current_dir(&clone).args([
            "commit",
            "-m",
            &format!("Add question #{id}: {}", sub.title),
        ]),
        "git commit",
    )?;
    progress("pushing…");
    run(
        Command::new("git").current_dir(&clone).args([
            "push",
            "--force-with-lease",
            "-u",
            "origin",
            &branch,
        ]),
        "git push",
    )?;
    progress("opening the pull request…");
    let head = if own {
        branch.clone()
    } else {
        format!("{login}:{branch}")
    };
    run(
        Command::new(gh).current_dir(&clone).args([
            "pr",
            "create",
            "--repo",
            REPO,
            "--base",
            &base,
            "--head",
            &head,
            "--title",
            &format!("Add question: {}", sub.title),
            "--body",
            &sub.body,
        ]),
        "gh pr create",
    )
}

/// Question ids claimed by open PRs (from the folders they add).
fn claimed_ids(gh: &Path) -> Vec<u32> {
    run(
        Command::new(gh).args([
            "pr",
            "list",
            "--repo",
            REPO,
            "--state",
            "open",
            "--limit",
            "200",
            "--json",
            "files",
            "--jq",
            ".[].files[].path",
        ]),
        "gh pr list",
    )
    .map(|out| ids_in_paths(&out))
    .unwrap_or_default()
}

/// Ids of `questions/NNNN-slug/...` folders in a list of paths.
fn ids_in_paths(paths: &str) -> Vec<u32> {
    paths
        .lines()
        .filter_map(|p| p.strip_prefix("questions/"))
        .filter_map(|rest| rest.split(['/', '-']).next()?.parse().ok())
        .collect()
}

/// One past the highest question id in a `questions/` directory.
pub fn next_id(questions: &Path) -> Result<u32> {
    let mut max = 0;
    for entry in
        std::fs::read_dir(questions).with_context(|| format!("reading {}", questions.display()))?
    {
        let name = entry?.file_name().to_string_lossy().into_owned();
        if let Some(n) = name.split('-').next().and_then(|n| n.parse::<u32>().ok()) {
            max = max.max(n);
        }
    }
    Ok(max + 1)
}

/// Copies a draft question folder, renumbering it to `id` (folder name,
/// `meta.json` and the boilerplate headers).
pub fn copy_question(src: &Path, dest: &Path, id: u32) -> Result<()> {
    std::fs::create_dir_all(dest)?;
    for entry in walk(src)? {
        let rel = entry.strip_prefix(src)?;
        let to = dest.join(rel);
        if let Some(parent) = to.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut body = std::fs::read_to_string(&entry)?;
        if rel == Path::new("meta.json") {
            let mut meta: serde_json::Value = serde_json::from_str(&body)?;
            meta["id"] = id.into();
            body = serde_json::to_string_pretty(&meta)? + "\n";
        } else if rel.starts_with("boilerplate") {
            body = renumber_header(&body, id);
        }
        std::fs::write(&to, body)?;
    }
    Ok(())
}

/// Rewrites the `<n>. Title` header on a boilerplate's first line.
fn renumber_header(code: &str, id: u32) -> String {
    let mut lines = code.lines();
    let Some(first) = lines.next() else {
        return code.to_string();
    };
    let prefix_len = first.find(|c: char| c.is_ascii_digit());
    let fixed = match prefix_len {
        Some(start) => {
            let digits = first[start..]
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .count();
            format!("{}{id}{}", &first[..start], &first[start + digits..])
        }
        None => first.to_string(),
    };
    let mut out = fixed;
    for l in lines {
        out.push('\n');
        out.push_str(l);
    }
    if code.ends_with('\n') {
        out.push('\n');
    }
    out
}

fn walk(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            out.extend(walk(&path)?);
        } else {
            out.push(path);
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::none("", &[])]
    #[case::one_pr("questions/0018-lis/meta.json\nquestions/0018-lis/tests.json", &[18, 18])]
    #[case::ignores_other_files("src/main.rs\nquestions/README.md\nquestions/0021-x/a.py", &[21])]
    fn finds_claimed_ids(#[case] paths: &str, #[case] expected: &[u32]) {
        assert_eq!(ids_in_paths(paths), expected);
    }

    #[rstest]
    #[case(
        "# 18. Word Ladder (hard)\n# hi\n",
        13,
        "# 13. Word Ladder (hard)\n# hi\n"
    )]
    #[case("// 0. Title (easy)\n", 7, "// 7. Title (easy)\n")]
    #[case("# 123. X\n# 1. 2.\n", 9, "# 9. X\n# 1. 2.\n")]
    #[case("no number here\n", 5, "no number here\n")]
    #[case("", 5, "")]
    fn renumbers_headers(#[case] code: &str, #[case] id: u32, #[case] expected: &str) {
        assert_eq!(renumber_header(code, id), expected);
    }

    #[test]
    fn renumbers_a_copied_question() {
        let tmp = tempfile::tempdir().unwrap();
        let questions = tmp.path().join("questions");
        std::fs::create_dir_all(questions.join("0007-a")).unwrap();
        std::fs::create_dir_all(questions.join("0012-b")).unwrap();
        assert_eq!(next_id(&questions).unwrap(), 13);

        let src = tmp.path().join("draft/0018-word-ladder");
        std::fs::create_dir_all(src.join("boilerplate")).unwrap();
        std::fs::write(
            src.join("meta.json"),
            r#"{"$schema": "x", "id": 18, "slug": "word-ladder"}"#,
        )
        .unwrap();
        std::fs::write(
            src.join("boilerplate/python.py"),
            "# 18. Word Ladder (hard)\n# hi 1. 2.\n",
        )
        .unwrap();
        let dest = questions.join("0013-word-ladder");
        copy_question(&src, &dest, 13).unwrap();
        let meta = std::fs::read_to_string(dest.join("meta.json")).unwrap();
        assert!(meta.contains("\"id\": 13"), "{meta}");
        assert!(
            meta.find("$schema").unwrap() < meta.find("\"id\"").unwrap(),
            "key order kept"
        );
        assert_eq!(
            std::fs::read_to_string(dest.join("boilerplate/python.py")).unwrap(),
            "# 13. Word Ladder (hard)\n# hi 1. 2.\n"
        );
    }
}
