//! The background music: tracks listed and served, and each page's mood.

use std::fs;
use std::path::Path;

use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use data::{GameData, InvestigatorId, OtherWorldId};
use engine::{Area, Place, Setup};
use tower::ServiceExt;

use crate::ambiance::{Ambiance, Screen};
use crate::database::Database;
use crate::music_library::MusicLibrary;
use crate::play_mode::PlayMode;
use crate::session::Session;
use crate::tests::other_world_pause::play_into_an_other_world;
use crate::tests::test_app::TestApp;

const NEW_GAME: &str = "investigateur=amanda_sharpe&ia=2&ia1=&ia2=&grand_ancien=&graine=7";

/// A music folder with one fake 1000-byte track in each of `folders`.
fn music_folder(root: &Path, folders: &[&str]) -> MusicLibrary {
    let music = root.join("musiques");
    for folder in folders {
        fs::create_dir_all(music.join(folder)).expect("folder");
        fs::write(music.join(folder).join("Piste (1).mp3"), [7u8; 1000]).expect("track");
    }
    MusicLibrary::scan(&music)
}

/// Sends `request` and returns the status and the `Content-Type`.
async fn status_and_type(app: &TestApp, request: Request<Body>) -> (StatusCode, Option<String>) {
    let response = app.router.clone().oneshot(request).await.expect("response");
    let kind = response
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned);
    (response.status(), kind)
}

fn mood_of(page: &str) -> &str {
    let start = page.find("data-ambiance=\"").expect("a mood") + "data-ambiance=\"".len();
    &page[start..start + page[start..].find('"').expect("closing quote")]
}

#[test]
fn a_mood_without_tracks_plays_the_default_one() {
    let dir = tempfile::tempdir().expect("temp dir");
    let library = music_folder(dir.path(), &["default"]);
    assert_eq!(library.playable(Ambiance::Combat), Ambiance::Default);
    assert_eq!(library.playable(Ambiance::OtherWorld), Ambiance::Default);
    assert_eq!(library.tracks[&Ambiance::Default], ["Piste (1).mp3"]);
    let nothing = MusicLibrary::scan(&dir.path().join("absent"));
    assert!(nothing.tracks.values().all(Vec::is_empty));
}

#[tokio::test(flavor = "current_thread")]
async fn tracks_are_listed_and_served_with_ranges() {
    let dir = tempfile::tempdir().expect("temp dir");
    let library = music_folder(dir.path(), &["default", "other_worlds"]);
    let app = TestApp::with_music(&dir.path().join("db"), library).await;

    let list = app.get("/musique/pistes").await.body;
    assert_eq!(
        list,
        r#"{"default":["Piste (1).mp3"],"other_world":["Piste (1).mp3"],"combat":[]}"#
    );

    let track = "/musique/other_world/Piste%20(1).mp3";
    let whole = Request::get(track).body(Body::empty()).expect("request");
    let (status, kind) = status_and_type(&app, whole).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(kind.as_deref(), Some("audio/mpeg"));
    let part = Request::get(track)
        .header(header::RANGE, "bytes=0-99")
        .body(Body::empty())
        .expect("request");
    assert_eq!(
        status_and_type(&app, part).await.0,
        StatusCode::PARTIAL_CONTENT
    );

    for refused in [
        "/musique/combat/Piste%20(1).mp3",
        "/musique/other_worlds/Piste%20(1).mp3",
        "/musique/default/..%2Fother_worlds%2FPiste%20(1).mp3",
        "/musique/default/../other_worlds/Piste%20(1).mp3",
    ] {
        let request = Request::get(refused).body(Body::empty()).expect("request");
        let status = status_and_type(&app, request).await.0;
        assert!(!status.is_success(), "{refused}: {status}");
    }
}

#[tokio::test(flavor = "current_thread")]
async fn each_page_carries_its_mood() {
    let dir = tempfile::tempdir().expect("temp dir");
    let all = ["default", "other_worlds", "combat"];
    let path = dir.path().join("db");
    let app = TestApp::with_music(&path, music_folder(dir.path(), &all)).await;
    let home = app.get("/").await.body;
    assert_eq!(mood_of(&home), "default");
    assert!(home.contains("<main id=\"contenu\""), "{home}");
    assert!(home.contains("id=\"musique\""), "{home}");
    app.post("/nouvelle", NEW_GAME).await;
    assert_eq!(mood_of(&app.get("/partie/1").await.body), "default");
    assert_eq!(mood_of(&app.get("/partie/1/journal").await.body), "default");
    assert_eq!(mood_of(&app.get("/partie/1/combat").await.body), "combat");

    let other = tempfile::tempdir().expect("temp dir");
    let path = other.path().join("db");
    let data = GameData::load(Path::new("../../data")).expect("data");
    let db = Database::open(&path).await.expect("db");
    play_into_an_other_world(&db, &data).await;
    db.close().await;
    let app = TestApp::with_music(&path, music_folder(other.path(), &all)).await;
    let pause = app.get("/partie/1/autre-monde").await.body;
    assert_eq!(mood_of(&pause), "other_world");
}

#[tokio::test(flavor = "current_thread")]
async fn standing_in_an_other_world_plays_its_music() {
    let dir = tempfile::tempdir().expect("temp dir");
    let db = Database::open(&dir.path().join("db")).await.expect("db");
    let data = GameData::load(Path::new("../../data")).expect("data");
    let setup = Setup {
        seed: 7,
        players: 1,
        investigators: vec![InvestigatorId::new("amanda_sharpe")],
        ancient_one: None,
    };
    let mut session = Session::start(&db, &data, setup, PlayMode::Solo)
        .await
        .expect("start");
    assert!(!session.in_combat());
    assert_eq!(Ambiance::of(&session, Screen::Other), Ambiance::Default);
    assert_eq!(Ambiance::of(&session, Screen::Combat), Ambiance::Combat);
    session.outcome.state.investigators[0].place = Place::OtherWorld {
        world: OtherWorldId::new("rlyeh"),
        area: Area::Second,
    };
    assert_eq!(Ambiance::of(&session, Screen::Other), Ambiance::OtherWorld);
    assert_eq!(Ambiance::of(&session, Screen::Combat), Ambiance::Combat);
    db.close().await;
}
