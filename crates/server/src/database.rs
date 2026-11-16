use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use polodb_core::{CollectionT, Database as PoloDatabase};
use serde::{Deserialize, Serialize};

const VISITS: &str = "visit";

/// Handle on the embedded PoloDB database.
///
/// Cheap to clone: all clones share the same storage, which is flushed and
/// unlocked when the last clone is dropped. PoloDB calls are blocking; they
/// are short enough to run directly on the single server thread.
#[derive(Clone)]
pub struct Database {
    db: PoloDatabase,
}

/// Database failure.
#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    /// Error reported by PoloDB.
    #[error("erreur de la base de données : {0}")]
    Polo(#[from] polodb_core::Error),
}

#[derive(Serialize, Deserialize)]
struct Visit {
    at_unix_secs: u64,
}

impl Database {
    /// Opens the database stored in the folder `path`, creating it if needed.
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::Polo`] if the storage cannot be opened.
    pub fn open(path: &Path) -> Result<Self, DatabaseError> {
        Ok(Self {
            db: PoloDatabase::open_path(path)?,
        })
    }

    /// Records one visit of the home page and returns the total visit count.
    ///
    /// Milestone J0 check that the server reads and writes the database.
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::Polo`] if the write or the count fails.
    pub fn record_visit(&self) -> Result<u64, DatabaseError> {
        let visits = self.db.collection::<Visit>(VISITS);
        // A clock before 1970 is not worth failing a page for: count it as 0.
        let at_unix_secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |elapsed| elapsed.as_secs());
        visits.insert_one(Visit { at_unix_secs })?;
        Ok(visits.count_documents()?)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use polodb_core::bson::doc;

    use super::*;

    #[derive(Serialize, Deserialize)]
    struct GameRow {
        name: String,
        state: String,
    }

    fn temp_database() -> (tempfile::TempDir, Database) {
        let dir = tempfile::tempdir().expect("temp dir");
        let db = Database::open(&dir.path().join("test.db")).expect("open database");
        (dir, db)
    }

    #[test]
    fn visits_are_counted() {
        let (_dir, db) = temp_database();
        assert_eq!(db.record_visit().expect("visit"), 1);
        assert_eq!(db.record_visit().expect("visit"), 2);
    }

    #[test]
    fn visits_survive_reopening() {
        let dir = tempfile::tempdir().expect("temp dir");
        let path = dir.path().join("test.db");
        let db = Database::open(&path).expect("open");
        db.record_visit().expect("visit");
        drop(db);
        let db = Database::open(&path).expect("reopen");
        assert_eq!(db.record_visit().expect("visit"), 2);
    }

    /// T4: a game state stored as a ~50 KB JSON string, rewritten on every move.
    #[test]
    fn large_state_blob_round_trip() {
        let (_dir, db) = temp_database();
        let games = db.db.collection::<GameRow>("game");
        let state = "x".repeat(50 * 1024);
        games
            .insert_one(GameRow {
                name: "test".to_owned(),
                state: String::new(),
            })
            .expect("insert");
        let start = Instant::now();
        for _ in 0..200 {
            games
                .update_one(
                    doc! { "name": "test" },
                    doc! { "$set": { "state": &state } },
                )
                .expect("update");
        }
        let elapsed = start.elapsed();
        let row = games
            .find_one(doc! { "name": "test" })
            .expect("read")
            .expect("row exists");
        assert_eq!(row.state, state);
        println!("200 writes of 50 KB: {elapsed:?}");
    }
}
