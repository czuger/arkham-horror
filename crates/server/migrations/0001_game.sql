-- Saved games: one row per game, its whole progress as JSON.
CREATE TABLE game (
    id INTEGER PRIMARY KEY,
    created_at TEXT NOT NULL
        DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
        CHECK (created_at GLOB '[0-9][0-9][0-9][0-9]-[0-9][0-9]-[0-9][0-9]T[0-9][0-9]:[0-9][0-9]:[0-9][0-9]Z'),
    title TEXT NOT NULL CHECK (length(title) <= 200),
    summary TEXT NOT NULL CHECK (length(summary) <= 200),
    over INTEGER NOT NULL CHECK (over IN (0, 1)),
    record TEXT NOT NULL CHECK (json_valid(record))
) STRICT;
