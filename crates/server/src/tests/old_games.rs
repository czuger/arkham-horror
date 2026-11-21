//! Games saved before a rules change.

use std::path::Path;

use data::{GameData, InvestigatorId};
use engine::Setup;

use crate::app_error::AppError;
use crate::database::Database;
use crate::play_mode::PlayMode;
use crate::session::{Session, delete_outdated_games};

fn setup() -> Setup {
    Setup {
        seed: 7,
        players: 3,
        investigators: vec![InvestigatorId::new("amanda_sharpe")],
        ancient_one: None,
    }
}

/// Saves a game with one more answer that is not an option.
async fn outdated_game(db: &Database, data: &GameData) -> i64 {
    let mut session = Session::start(db, data, setup(), PlayMode::Solo)
        .await
        .expect("start");
    session.progress.answers.push(99);
    session.save(db, data).await.expect("save");
    session.id
}

/// Opening a game whose answers no longer fit the rules deletes it.
#[tokio::test(flavor = "current_thread")]
async fn an_outdated_game_is_deleted_when_opened() {
    let dir = tempfile::tempdir().expect("temp dir");
    let db = Database::open(&dir.path().join("db")).await.expect("db");
    let data = GameData::load(Path::new("../../data")).expect("data");
    let id = outdated_game(&db, &data).await;
    let opened = Session::load(&db, &data, id).await;
    assert!(matches!(opened, Err(AppError::Outdated)));
    assert!(db.game(id).await.expect("read").is_none());
    db.close().await;
}

/// At startup, the outdated games are deleted and the others kept.
#[tokio::test(flavor = "current_thread")]
async fn outdated_games_are_deleted_at_startup() {
    let dir = tempfile::tempdir().expect("temp dir");
    let db = Database::open(&dir.path().join("db")).await.expect("db");
    let data = GameData::load(Path::new("../../data")).expect("data");
    let good = Session::start(&db, &data, setup(), PlayMode::Solo)
        .await
        .expect("start")
        .id;
    let bad = outdated_game(&db, &data).await;
    assert_eq!(delete_outdated_games(&db, &data).await.expect("purge"), 1);
    assert!(db.game(good).await.expect("read").is_some());
    assert!(db.game(bad).await.expect("read").is_none());
    db.close().await;
}
