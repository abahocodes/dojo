# dojo — plan

A fullscreen terminal app for coding-interview practice. Run `dojo`, type slash
commands (`/solve 42`, `/need 3`, `/report`). Solve in your own editor; dojo runs
the tests, keeps time, tracks every attempt and models where you're strong and
where you have gaps.

## Decisions

| Area | Choice |
|---|---|
| Language | Rust (edition 2024) |
| UI | ratatui fullscreen (alternate screen), crossterm backend. No inline mode. |
| Storage | SQLite via sqlx (bundled SQLite — no user-installed deps). **All SQL uses sqlx's compile-time checked macros**: `query_as!` for rows, `query!` for statements without rows. Migrations in `migrations/` (`sqlx::migrate!`). |
| Questions | Self-contained asset folders with a schema, embedded in the binary (`rust-embed`). See `questions/README.md`. JSON Schemas in `schema/` are generated from the serde types (`dojo schema`). |
| Solve languages | Python 3 and JavaScript (Node). A language = a harness in `src/runner/` + `boilerplate/` and `solutions/` files in every question. |
| Content updates | Ship with releases (revisit `/update` packs later). |
| Platforms | macOS (arm64, x86_64), Linux (musl static). Released with cargo-dist. |

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
file starts as a copy of the question's boilerplate; finishing an attempt saves
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
solution). Questions are not versioned; attempts record the dojo release.
Each question folder has its own generated `cargo test`.

## App

Layout: header · scrollable transcript · input with slash autocomplete · status bar
(timer, question, progress, hints, runs, language, editor).

Commands (a deliberately tight set; the prompt always suggests the next one):

| Area | Commands |
|---|---|
| Practice | `/solve <id \| query \| random> [-n N]` (always fresh; `/solve need` in M4) |
| On a question | `/test`, `/submit`, `/hint`, `/solution`, `/edit`, `/pause`, `/skip`, `/next` |
| Browse | `/list`, `/show`, `/past` (M3) |
| Insight | `/report` (M4, includes history) |
| Settings | `/editor`, `/lang`, `/config` |
| App | `/copy`, `/clear`, `/help`, `/quit` (`/exit`, `/q`), `/contribute` (M6) |

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

Tables: `sessions`, `attempts`, `events` (source of truth, JSON payloads),
`review_state` (FSRS), `input_history`. Stats are derived from events so the
competence model can change without migrations.

## Competence model

- Attempt grade 0–1: pass base, minus failed runs, hint depth, time over target.
  Revealed ≈ 0.1, skip/fail = 0.
- FSRS-style scheduling per question → due reviews.
- Tag mastery: recency-decayed, difficulty-weighted grade average with a
  confidence measure by attempt count.
- `/need` score: due review + tag weakness + unseen-in-weak-tag − recently seen,
  ramping difficulty with mastery.

## Contribute

Token in OS keychain (env vars override) → wizard (idea, constraints, examples,
difficulty, tags, companies) → LLM drafts the full package → local validation
(schema, reference solution passes all tests, dedupe) → user edits in their editor →
originality attestation → PR via `gh` or GitHub device flow → repo CI re-validates →
ships in the next release.

## Milestones

| # | Scope | Status |
|---|---|---|
| M0 | Cargo project, question schema, `dojo validate`, seed questions, Python adapter | done (17 questions) |
| M1 | Fullscreen shell: layout, transcript, input + autocomplete, status bar, markdown, `/help` `/list` `/show` | done |
| M2 | Session core: `/solve <id>`, editor launch/suspend, timer, `/test` `/submit` `/hint` `/solution` `/skip`, SQLite recording | done (also multi-question queues, `/next`, `/giveup`, `/pause`, save-triggered tests) |
| M3 | end-of-session review, `/past`, idle detection | `/solve random` done |
| M4 | Grading, FSRS, mastery, `/need`, tabbed `/report` | |
| M5 | More languages (Go, TypeScript, Rust, Java, C++) | Python + JavaScript done |
| M6 | `/contribute` + repo CI | |
| M7 | cargo-dist releases, Homebrew tap | |
