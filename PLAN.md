# dojo — plan

A fullscreen terminal app for coding-interview practice. Run `dojo`, type slash
commands (`/solve 42`, `/solve need 3`, `/report`). Solve in your own editor; dojo runs
the tests, keeps time, tracks every attempt and models where you're strong and
where you have gaps.

## Decisions

| Area | Choice |
|---|---|
| Language | Rust (edition 2024) |
| UI | ratatui fullscreen (alternate screen), crossterm backend. No inline mode. |
| Storage | SQLite via sqlx (bundled SQLite — no user-installed deps). **All SQL uses sqlx's compile-time checked macros**: `query_as!` for rows, `query!` for statements without rows. Migrations in `migrations/` (`sqlx::migrate!`). |
| Questions | Self-contained asset folders with a schema, packed by `build.rs` into one zstd-compressed blob in the binary. 500 questions; ids 501–1000 are planned in `questions/CURRICULUM.md` (about 25% easy, 55% medium, 20% hard). See `questions/README.md`. JSON Schemas in `schema/` are generated from the serde types (`dojo schema`). |
| Solve languages | Python 3, JavaScript (Node), TypeScript (Node), Java, C++ and Go; every question has all six. A language = a harness in `src/runner/` (compiled languages get a per-question driver, `src/runner/compiled/`) + `boilerplate/` and `solutions/` files in every question. |
| Content updates | Ship with releases (revisit `/update` packs later). |
| Platforms | macOS (arm64, x86_64), Linux (arm64, x86_64). Released with cargo-dist: shell installer, GitHub Releases, Homebrew (`brew install abahocodes/tap/dojo`). |

## Development

- `scripts/dev-db.sh` builds `target/dev.db` from `migrations/`; `.env` points
  `DATABASE_URL` at it so the sqlx macros validate SQL during `cargo build`.
- After changing SQL or migrations: `scripts/dev-db.sh --prepare` and commit
  `.sqlx/`. CI and release builds use `SQLX_OFFLINE=true`.
- `dojo validate` checks every question (CI runs it too).

## Paths

- config: `~/.config/dojo/config.toml`
- data: `~/.local/share/dojo/dojo.db` — sessions, attempts (with code), events
- work: `~/.local/state/dojo/work/<NNNN-slug>/<lang>/solution.<ext>` — scratch
  file for the attempt in progress (override with `workspace` in config)

A **session** is a period of practice, recorded only in the database. A work
file starts as the question's boilerplate with the problem statement as
comments on top (a terminal editor hides dojo's screen); finishing an attempt saves
its code to the database and deletes the work folder.

Closing dojo mid-question (`/quit`, Ctrl+C twice, Ctrl+D, a closed terminal)
pauses the attempt: it stays open with its clock, counters and code (the clock
is autosaved every 5s). Next time, Enter on the welcome screen or
`/edit` continues it — same attempt, same timer, same language, editor
reopened at the last line changed. `/solve <id>` starts over instead (the old
attempt is skipped, its code kept).

Keys: Ctrl+C cancels a test run, then clears input, then closes suggestions;
twice on an empty prompt quits. Ctrl+D on an empty prompt quits. Enter on an
empty prompt does the obvious next thing (continue, back to the editor,
submit, next question).

## Question format

See `questions/README.md`. `meta.json` is the manifest: metadata plus the path of every other file
(statement, hints, explanation, tests, and per-language boilerplate +
solution). Questions are not versioned; they are edited in place.
Each question folder has its own generated `cargo test` (behind
`--features question-tests`).

## App

Layout: header · scrollable transcript · input with slash autocomplete · status bar
(timer, question, progress, hints, runs, language, editor).

Commands (a deliberately tight set; the prompt always suggests the next one):

| Area | Commands |
|---|---|
| Practice | `/solve <id \| query \| need \| random> [N]` (always fresh) |
| On a question | `/test`, `/submit`, `/hint`, `/solution`, `/edit`, `/pause`, `/skip`, `/next` |
| Browse | `/list`, `/show`, `/past [id] [n]` |
| Insight | `/report [tag \| company \| difficulty]` (includes history) |
| Settings | `/editor`, `/lang`, `/config` |
| App | `/copy`, `/clear`, `/donate`, `/help`, `/quit` (`/exit`, `/q`), `/contribute`, `/accept` |

First run (no config file) walks through picking an editor and a language,
then saves the config. Idle pause: no key, scroll or save for
`idle_pause_minutes` (default 15) pauses the timer, and the idle time isn't
counted. Closing dojo shows the end-of-session review (each question, outcome,
time) and prints it to the normal terminal.

A **session** is the period the dojo window is open: it starts with the first
question and ends when dojo closes. `/solve` always starts fresh. `/edit` is
how you get back to work: it reopens the editor (restarting a paused timer),
or with nothing open continues unfinished work (`/edit <id>` picks which). `/skip` moves on: recorded as a fail if tests
were run, a skip otherwise; `/solution` still works afterwards. Anything not
completed when something else starts is skipped the same way: `/solve` mid-
question skips the open question (including `/solve` of the same question,
which starts it over), and `/solve` of a question left unfinished in an earlier
window skips that attempt.

CLI passthrough: `dojo solve 42` opens the app and runs it; `dojo report --json`
and `dojo validate` run headless.

Terminal editors (vim/nvim/emacs -nw/helix/nano) suspend the app (leave alternate
screen), timer keeps running. GUI editors keep the app live; saving the file re-runs
visible tests.

Fullscreen tradeoffs handled: own transcript buffer (PgUp/PgDn, wheel, g/G),
structured blocks re-rendered on resize, `/copy` via OSC 52, session summary printed
to the normal terminal on exit.

## Data model

One migration, `migrations/20261007000000_initial.sql`. Tables: `sessions`,
`attempts` (one try at one question: language, outcome, counters, and
`active_secs`, the time the user took, which feeds the grade), `events` (what
happened during an attempt, JSON payloads) and `input_history`.

- **Enums:** TEXT columns limited by CHECK; read in Rust as enums via
  `sqlx::Type`: `Language`, `Outcome`, `EventKind`.
- **Timestamps:** DATETIME columns, read and written in Rust as
  `time::OffsetDateTime` and converted to `jiff::Timestamp` at the store
  boundary.
- **Constraints:** non-negative counters, `failed_runs <= test_runs`, an
  outcome iff an end time, end after start, JSON payloads, foreign keys with
  cascade.
- **No question versions:** questions are edited in place.

## Competence model

(`src/model.rs`, pure and unit-tested; computed on demand from attempts.)

- Attempt grade 0–1: a pass starts at 1, minus 0.08 per failed run (max 4),
  up to 0.4 for hints used, up to 0.25 for time over target; floor 0.35.
  Revealed = 0.15, fail/skip = 0.
- Question score: recency-weighted grade average (half-life 30 days; a skip
  without tests weighs half).
- Topic mastery: difficulty-weighted mean over attempted questions, with
  confidence from the number of distinct questions tried. Company readiness:
  same over all its questions, untried counting as 0.
- Suggestions: gap topics (< 60%) first, then due reviews (solved, not
  cleanly, a week+ ago), then never-practiced topics.
- Selection lives in `src/select/`: `/solve` words parse into one
  `Request` whatever their order (`google need 3` = `need google 3`), a
  `Filter` (tags, companies, difficulties) narrows the pool, and a
  `Strategy` (`Need`, `Random`) orders it. Questions and history come
  through a `Library` trait and chance through `Dice`, so tests use mocks.
- Filter words alone (`/solve google`) mean `need` within them; free text
  (`/solve two sum`) searches titles.
- `Need` order: gap topics, due reviews, unpracticed topics, then unsolved
  questions (ones tried in the last day after), then the weakest solved
  ones. Untried questions come mediums first (asked most), then easies,
  then hards.

## Contribute

`/contribute` (or `dojo contribute`): describe a question → dojo drafts it
in all six languages with Claude (`claude-opus-5-5` by default), OpenAI or a
local model through Ollama, using the contributor's API key (OS keychain; `ANTHROPIC_API_KEY` / `OPENAI_API_KEY` override) →
writes the asset folder, runs an optional stress-case generator, computes
expected outputs by running the Python reference solution, and runs the full
validator; problems go back to the model (up to 3 rounds). The draft shows in
the window; typing a change revises it (append-only conversation), `/accept`
opens the PR via `gh` (offering to install it with brew/apt/dnf/pacman/zypper
or from GitHub releases into `~/.local/bin`, and running `gh auth login` if
needed): fork (or clone when you own the repo) → branch from upstream →
renumber to the next free id → commit → push → `gh pr create`. Drafts live in
`~/.local/state/dojo/contrib/<id>/` and resume after a restart.

Nothing is submitted unless it passes locally: `/accept` refuses a draft
with problems, re-validates the folder as it is on disk (it may have been
edited by hand), and validates the renumbered folder again inside the repo
checkout against the current bank before committing.

Two separate workflows:
- `ci.yml`, for dojo itself (any change outside `questions/`): fmt, clippy
  and dojo's tests on Linux and macOS. It does not re-run the bank: the
  runner is tested per language on small fixture questions
  (`runner::language_tests`), so CI grows with languages, not questions.
  The per-question tests exist behind `--features question-tests`.
- `question.yml`, for contributed questions (PRs touching `questions/`):
  builds dojo and runs `dojo validate --question <folder>` on only the
  folders the PR adds or changes (split into up to 8 parallel shards), plus
  the id/slug uniqueness check. Validation checks the schema and that the
  question can't break the runner.
- `release.yml` (cargo-dist): a `v*` tag builds every platform, publishes the
  GitHub Release and pushes the Homebrew formula with a deploy key
  (`HOMEBREW_TAP_DEPLOY_KEY`). Release by bumping the version in a PR, then
  tagging the merge.

## Milestones

| # | Scope | Status |
|---|---|---|
| M0 | Cargo project, question schema, `dojo validate`, seed questions, Python adapter | done (now 500 questions) |
| M1 | Fullscreen shell: layout, transcript, input + autocomplete, status bar, markdown, `/help` `/list` `/show` | done |
| M2 | Session core: `/solve <id>`, editor launch/suspend, timer, `/test` `/submit` `/hint` `/solution` `/skip`, SQLite recording | done (also multi-question queues, `/next`, `/giveup`, `/pause`, save-triggered tests) |
| M3 | End-of-session review, idle detection | done (also `/solve random`, `/past`, first-run setup) |
| M4 | Grading, mastery, tabbed `/report`, `dojo report --json`, `/solve need` | done |
| M5 | More languages | done: TypeScript, Java, C++, Go (Rust dropped) |
| M6 | `/contribute` + repo CI | done (the fork → PR flow hasn't been run end to end by an outside contributor) |
| M7 | cargo-dist releases, Homebrew tap | done (v0.3.0) |
| M8 | Bank to 1000 questions | 500 done; 501–1000 planned |

Open: run `/contribute` end to end as an outside contributor; `/update`
question packs if releases stop being enough.
