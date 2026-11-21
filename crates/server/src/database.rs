use std::path::{Path, PathBuf};
use std::time::Duration;

use sqlx::SqlitePool;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous};

use crate::game_row::{GameContent, GameRow};
use crate::schema;

/// How long a connection waits for a lock before failing.
const BUSY_TIMEOUT: Duration = Duration::from_secs(5);

/// Read connections open at most at once.
const READERS: u32 = 4;

/// Handle on the SQLite database file.
///
/// SQLite allows one writer at a time: every write goes through a pool of
/// one connection, every read through a separate read-only pool. Cheap to
/// clone: clones share both pools. Call [`Database::close`] before exiting
/// so the write-ahead log is folded back into the file.
#[derive(Clone)]
pub struct Database {
    writer: SqlitePool,
    reader: SqlitePool,
}

/// Database failure.
#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    /// Error reported by SQLite or sqlx.
    #[error("erreur de la base de données : {0}")]
    Sqlx(#[from] sqlx::Error),
    /// The path is a folder, most likely the old PoloDB database.
    #[error(
        "« {} » est un dossier (sans doute l'ancienne base PoloDB) : déplacez-le, ou choisissez un autre fichier avec ARKHAM_DB",
        .0.display()
    )]
    Folder(PathBuf),
    /// The file was written by a newer version of the program.
    #[error("base de données trop récente : schéma {found}, ce programme connaît {known}")]
    Newer {
        /// Schema version found in the file.
        found: i64,
        /// Schema versions this program knows.
        known: usize,
    },
}

impl Database {
    /// Opens the database file `path`, creating it and its tables if needed.
    ///
    /// # Errors
    ///
    /// Returns a [`DatabaseError`] if the path is a folder, the file cannot
    /// be opened or its schema cannot be brought up to date.
    pub async fn open(path: &Path) -> Result<Self, DatabaseError> {
        if path.is_dir() {
            return Err(DatabaseError::Folder(path.to_owned()));
        }
        // The writer stays open for good: it creates the file, and keeps
        // the write-ahead log files that read-only connections need.
        let writer = SqlitePoolOptions::new()
            .max_connections(1)
            .min_connections(1)
            .idle_timeout(None)
            .max_lifetime(None)
            .connect_with(options(path).create_if_missing(true))
            .await?;
        schema::migrate(&writer).await?;
        let reader = SqlitePoolOptions::new()
            .max_connections(READERS)
            .connect_with(options(path).read_only(true))
            .await?;
        Ok(Self { writer, reader })
    }

    /// Closes every connection, waiting for those in use.
    pub async fn close(&self) {
        self.reader.close().await;
        self.writer.close().await;
    }

    /// Saves a new game.
    ///
    /// # Returns
    ///
    /// The game's number: one more than the highest so far.
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::Sqlx`] if the write fails.
    pub async fn insert_game(&self, game: &GameContent) -> Result<i64, DatabaseError> {
        let id = sqlx::query_scalar(
            "INSERT INTO game (title, summary, over, record) VALUES (?, ?, ?, ?) RETURNING id",
        )
        .bind(game.title.as_str())
        .bind(game.summary.as_str())
        .bind(game.over)
        .bind(game.record.as_str())
        .fetch_one(&self.writer)
        .await?;
        Ok(id)
    }

    /// The game with this number, if any.
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::Sqlx`] if the read fails.
    pub async fn game(&self, id: i64) -> Result<Option<GameRow>, DatabaseError> {
        Ok(sqlx::query_as(
            "SELECT id, created_at, title, summary, over, record FROM game WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.reader)
        .await?)
    }

    /// Every game, newest first.
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::Sqlx`] if the read fails.
    pub async fn all_games(&self) -> Result<Vec<GameRow>, DatabaseError> {
        Ok(sqlx::query_as(
            "SELECT id, created_at, title, summary, over, record FROM game ORDER BY id DESC",
        )
        .fetch_all(&self.reader)
        .await?)
    }

    /// Overwrites a saved game; does nothing if there is no such game.
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::Sqlx`] if the write fails.
    pub async fn update_game(&self, id: i64, game: &GameContent) -> Result<(), DatabaseError> {
        sqlx::query("UPDATE game SET title = ?, summary = ?, over = ?, record = ? WHERE id = ?")
            .bind(game.title.as_str())
            .bind(game.summary.as_str())
            .bind(game.over)
            .bind(game.record.as_str())
            .bind(id)
            .execute(&self.writer)
            .await?;
        Ok(())
    }

    /// Deletes a saved game.
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::Sqlx`] if the write fails.
    pub async fn delete_game(&self, id: i64) -> Result<(), DatabaseError> {
        sqlx::query("DELETE FROM game WHERE id = ?")
            .bind(id)
            .execute(&self.writer)
            .await?;
        Ok(())
    }
}

/// Connection settings shared by both pools.
fn options(path: &Path) -> SqliteConnectOptions {
    SqliteConnectOptions::new()
        .filename(path)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal)
        .foreign_keys(true)
        .busy_timeout(BUSY_TIMEOUT)
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;

    async fn temp_database() -> (tempfile::TempDir, Database) {
        let dir = tempfile::tempdir().expect("temp dir");
        let db = Database::open(&dir.path().join("test.db"))
            .await
            .expect("open database");
        (dir, db)
    }

    fn content(n: i64) -> GameContent {
        GameContent {
            title: format!("partie {n}"),
            summary: String::new(),
            over: false,
            record: "{}".to_owned(),
        }
    }

    #[tokio::test(flavor = "current_thread")]
    async fn games_are_numbered_and_listed_newest_first() {
        let (_dir, db) = temp_database().await;
        assert_eq!(db.insert_game(&content(1)).await.expect("insert"), 1);
        assert_eq!(db.insert_game(&content(2)).await.expect("insert"), 2);
        let games = db.all_games().await.expect("list");
        let ids: Vec<i64> = games.iter().map(|r| r.id).collect();
        assert_eq!(ids, vec![2, 1]);
        assert!(games[0].created_at.ends_with('Z'));
        db.delete_game(2).await.expect("delete");
        assert_eq!(db.insert_game(&content(3)).await.expect("insert"), 2);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn games_survive_reopening() {
        let dir = tempfile::tempdir().expect("temp dir");
        let path = dir.path().join("test.db");
        let db = Database::open(&path).await.expect("open");
        let id = db.insert_game(&content(1)).await.expect("insert");
        let mut changed = content(1);
        changed.record = "[1,2,3]".to_owned();
        changed.over = true;
        db.update_game(id, &changed).await.expect("update");
        db.close().await;
        let db = Database::open(&path).await.expect("reopen");
        let row = db.game(id).await.expect("read").expect("row exists");
        assert_eq!((row.over, row.record.as_str()), (true, "[1,2,3]"));
        assert_eq!(db.game(9).await.expect("read"), None);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn the_schema_refuses_bad_values() {
        let (_dir, db) = temp_database().await;
        let mut bad_json = content(1);
        bad_json.record = "{".to_owned();
        assert!(db.insert_game(&bad_json).await.is_err());
        let mut long_title = content(1);
        long_title.title = "x".repeat(201);
        assert!(db.insert_game(&long_title).await.is_err());
        assert!(db.all_games().await.expect("list").is_empty());
    }

    #[tokio::test(flavor = "current_thread")]
    async fn a_folder_is_refused() {
        let dir = tempfile::tempdir().expect("temp dir");
        let error = Database::open(dir.path()).await.err().expect("refused");
        assert!(matches!(error, DatabaseError::Folder(_)));
    }

    /// T4: a game state stored as a ~50 KB JSON string, rewritten on every move.
    #[tokio::test(flavor = "current_thread")]
    async fn large_state_round_trip() {
        let (_dir, db) = temp_database().await;
        let id = db.insert_game(&content(1)).await.expect("insert");
        let mut big = content(1);
        big.record = format!("\"{}\"", "x".repeat(50 * 1024));
        let start = Instant::now();
        for _ in 0..200 {
            db.update_game(id, &big).await.expect("update");
        }
        let elapsed = start.elapsed();
        let row = db.game(id).await.expect("read").expect("row exists");
        assert_eq!(row.record, big.record);
        println!("200 writes of 50 KB: {elapsed:?}");
    }
}
