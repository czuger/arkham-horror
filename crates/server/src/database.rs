use std::path::Path;

use polodb_core::bson::doc;
use polodb_core::{CollectionT, Database as PoloDatabase};

use crate::game_row::GameRow;

/// Collection of the saved games.
const GAMES: &str = "partie";

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

    fn games(&self) -> polodb_core::Collection<GameRow> {
        self.db.collection::<GameRow>(GAMES)
    }

    /// Number for a new game: one more than the highest so far.
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::Polo`] if the read fails.
    pub fn next_game_id(&self) -> Result<i64, DatabaseError> {
        let last = self
            .games()
            .find(doc! {})
            .sort(doc! { "id": -1 })
            .run()?
            .next()
            .transpose()?;
        Ok(last.map_or(1, |row| row.id + 1))
    }

    /// Saves a new game.
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::Polo`] if the write fails.
    pub fn insert_game(&self, row: &GameRow) -> Result<(), DatabaseError> {
        self.games().insert_one(row)?;
        Ok(())
    }

    /// The game with this number, if any.
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::Polo`] if the read fails.
    pub fn game(&self, id: i64) -> Result<Option<GameRow>, DatabaseError> {
        Ok(self.games().find_one(doc! { "id": id })?)
    }

    /// Every game, newest first.
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::Polo`] if the read fails.
    pub fn all_games(&self) -> Result<Vec<GameRow>, DatabaseError> {
        let rows = self
            .games()
            .find(doc! {})
            .sort(doc! { "id": -1 })
            .run()?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    /// Overwrites a saved game (found by its number).
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::Polo`] if the write fails.
    pub fn update_game(&self, row: &GameRow) -> Result<(), DatabaseError> {
        self.games().update_one(
            doc! { "id": row.id },
            doc! { "$set": {
                "title": &row.title,
                "summary": &row.summary,
                "over": row.over,
                "record": &row.record,
            } },
        )?;
        Ok(())
    }

    /// Deletes a saved game.
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::Polo`] if the write fails.
    pub fn delete_game(&self, id: i64) -> Result<(), DatabaseError> {
        self.games().delete_one(doc! { "id": id })?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Serialize, Deserialize)]
    struct BlobRow {
        name: String,
        state: String,
    }

    fn temp_database() -> (tempfile::TempDir, Database) {
        let dir = tempfile::tempdir().expect("temp dir");
        let db = Database::open(&dir.path().join("test.db")).expect("open database");
        (dir, db)
    }

    fn row(id: i64) -> GameRow {
        GameRow {
            id,
            created_unix: 0,
            title: format!("partie {id}"),
            summary: String::new(),
            over: false,
            record: "{}".to_owned(),
        }
    }

    #[test]
    fn games_are_numbered_and_listed_newest_first() {
        let (_dir, db) = temp_database();
        assert_eq!(db.next_game_id().expect("id"), 1);
        db.insert_game(&row(1)).expect("insert");
        db.insert_game(&row(2)).expect("insert");
        assert_eq!(db.next_game_id().expect("id"), 3);
        let ids: Vec<i64> = db.all_games().expect("list").iter().map(|r| r.id).collect();
        assert_eq!(ids, vec![2, 1]);
        db.delete_game(2).expect("delete");
        assert_eq!(db.next_game_id().expect("id"), 2);
    }

    #[test]
    fn games_survive_reopening() {
        let dir = tempfile::tempdir().expect("temp dir");
        let path = dir.path().join("test.db");
        let db = Database::open(&path).expect("open");
        db.insert_game(&row(1)).expect("insert");
        let mut changed = row(1);
        changed.record = "[1,2,3]".to_owned();
        changed.over = true;
        db.update_game(&changed).expect("update");
        drop(db);
        let db = Database::open(&path).expect("reopen");
        assert_eq!(db.game(1).expect("read"), Some(changed));
        assert_eq!(db.game(9).expect("read"), None);
    }

    /// T4: a game state stored as a ~50 KB JSON string, rewritten on every move.
    #[test]
    fn large_state_blob_round_trip() {
        let (_dir, db) = temp_database();
        let games = db.db.collection::<BlobRow>("game");
        let state = "x".repeat(50 * 1024);
        games
            .insert_one(BlobRow {
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
