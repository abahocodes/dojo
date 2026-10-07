CREATE TABLE sessions (
    id          INTEGER PRIMARY KEY,
    started_at  TEXT NOT NULL,
    ended_at    TEXT,
    mode        TEXT NOT NULL,
    query       TEXT
);

CREATE TABLE attempts (
    id               INTEGER PRIMARY KEY,
    session_id       INTEGER NOT NULL REFERENCES sessions(id),
    question_id      INTEGER NOT NULL,
    question_version INTEGER NOT NULL,
    language         TEXT NOT NULL,
    started_at       TEXT NOT NULL,
    ended_at         TEXT,
    active_secs      INTEGER NOT NULL DEFAULT 0,
    outcome          TEXT,
    test_runs        INTEGER NOT NULL DEFAULT 0,
    failed_runs      INTEGER NOT NULL DEFAULT 0,
    hints_used       INTEGER NOT NULL DEFAULT 0,
    solution_viewed  BOOLEAN NOT NULL DEFAULT FALSE,
    code             TEXT
);

-- Source of truth for stats; payload is JSON.
CREATE TABLE events (
    id          INTEGER PRIMARY KEY,
    attempt_id  INTEGER NOT NULL REFERENCES attempts(id),
    at          TEXT NOT NULL,
    kind        TEXT NOT NULL,
    payload     TEXT
);

CREATE TABLE review_state (
    question_id INTEGER PRIMARY KEY,
    stability   REAL,
    difficulty  REAL,
    due_at      TEXT,
    last_grade  REAL
);

CREATE TABLE input_history (
    id    INTEGER PRIMARY KEY,
    at    TEXT NOT NULL,
    line  TEXT NOT NULL
);

CREATE INDEX attempts_question ON attempts(question_id);
CREATE INDEX events_attempt ON events(attempt_id);
