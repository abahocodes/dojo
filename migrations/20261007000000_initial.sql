-- dojo's database: one SQLite file per user.
--
-- SQLite has no enum or timestamp types, so:
--   * enums are TEXT columns limited to their values with CHECK; the Rust
--     side reads them as enums (sqlx::Type);
--   * timestamps are DATETIME columns holding UTC times, which sqlx reads and
--     writes as time::OffsetDateTime.
-- Questions aren't versioned: they're edited in place, so attempts only
-- record which question they were on.

-- A session: the time dojo was open and practicing.
CREATE TABLE sessions (
    id          INTEGER PRIMARY KEY,
    started_at  DATETIME NOT NULL,
    ended_at    DATETIME,
    CHECK (ended_at IS NULL OR ended_at >= started_at)
);

-- One try at one question. An attempt without an outcome is still open and
-- can be continued.
CREATE TABLE attempts (
    id               INTEGER PRIMARY KEY,
    session_id       INTEGER NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
    question_id      INTEGER NOT NULL CHECK (question_id > 0),
    language         TEXT NOT NULL
                     CHECK (language IN ('python', 'javascript', 'typescript', 'java', 'cpp', 'go')),
    started_at       DATETIME NOT NULL,
    ended_at         DATETIME,
    -- How long the user took: time with the timer running (pauses and time
    -- away excluded). Compared with the question's target time, it factors
    -- into the attempt's grade.
    active_secs      INTEGER NOT NULL DEFAULT 0 CHECK (active_secs >= 0),
    outcome          TEXT CHECK (outcome IN ('pass', 'revealed', 'fail', 'skip')),
    test_runs        INTEGER NOT NULL DEFAULT 0 CHECK (test_runs >= 0),
    failed_runs      INTEGER NOT NULL DEFAULT 0 CHECK (failed_runs BETWEEN 0 AND test_runs),
    hints_used       INTEGER NOT NULL DEFAULT 0 CHECK (hints_used >= 0),
    solution_viewed  BOOLEAN NOT NULL DEFAULT FALSE CHECK (solution_viewed IN (0, 1)),
    -- The user's code when they last stopped working on it.
    code             TEXT,
    CHECK (ended_at IS NULL OR ended_at >= started_at),
    -- Finished attempts have an outcome and an end; open ones have neither.
    CHECK ((outcome IS NULL) = (ended_at IS NULL))
);

CREATE INDEX attempts_question ON attempts(question_id);
CREATE INDEX attempts_session ON attempts(session_id);
-- Open attempts, looked up at startup.
CREATE INDEX attempts_open ON attempts(question_id, language) WHERE outcome IS NULL;

-- What happened during an attempt, in order. The source of truth behind the
-- counters on attempts.
CREATE TABLE events (
    id          INTEGER PRIMARY KEY,
    attempt_id  INTEGER NOT NULL REFERENCES attempts(id) ON DELETE CASCADE,
    at          DATETIME NOT NULL,
    kind        TEXT NOT NULL CHECK (kind IN (
                    'start', 'continue', 'editor_open', 'test_run', 'hint',
                    'solution_viewed', 'pause', 'idle_pause', 'resume',
                    'suspend', 'outcome'
                )),
    payload     TEXT CHECK (payload IS NULL OR json_valid(payload))
);

CREATE INDEX events_attempt ON events(attempt_id);

-- Lines typed at dojo's prompt, for history (↑/↓).
CREATE TABLE input_history (
    id    INTEGER PRIMARY KEY,
    at    DATETIME NOT NULL,
    line  TEXT NOT NULL CHECK (line <> '')
);
