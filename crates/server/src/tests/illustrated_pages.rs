//! The illustrations: served with a fingerprint, shown on the scene, the map
//! and the combat page.

use std::path::Path;

use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use data::{GameData, InvestigatorId};
use engine::{Choice, DecisionKind, Setup};
use tower::ServiceExt;

use crate::database::Database;
use crate::illustrations::{IllustrationSize, illustration_url};
use crate::play_mode::PlayMode;
use crate::routes::game_page;
use crate::session::Session;
use crate::tests::test_app::TestApp;

fn temp() -> tempfile::TempDir {
    tempfile::tempdir().expect("temp dir")
}

/// Gets `path` and returns the status and the `Cache-Control` header.
async fn cache_of(app: &TestApp, path: &str) -> (StatusCode, Option<String>) {
    let request = Request::get(path).body(Body::empty()).expect("request");
    let response = app.router.clone().oneshot(request).await.expect("response");
    let kind = response.headers().get(header::CONTENT_TYPE);
    if response.status() == StatusCode::OK {
        assert_eq!(kind.and_then(|k| k.to_str().ok()), Some("image/jpeg"));
    }
    let cache = response
        .headers()
        .get(header::CACHE_CONTROL)
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned);
    (response.status(), cache)
}

#[tokio::test(flavor = "current_thread")]
async fn illustrations_are_served_shrunk_and_cacheable() {
    let dir = temp();
    let app = TestApp::open(&dir.path().join("db")).await;
    for size in [IllustrationSize::Large, IllustrationSize::Thumbnail] {
        let url = illustration_url(size, "autres mondes/rlyeh_2.png");
        assert!(url.contains("autres%20mondes/rlyeh_2.jpg?v="), "{url}");
        let (status, cache) = cache_of(&app, &format!("/{url}")).await;
        assert_eq!(status, StatusCode::OK, "{url}");
        assert!(cache.is_some_and(|c| c.contains("immutable")));
        let bare = url.split('?').next().expect("path");
        let (status, cache) = cache_of(&app, &format!("/{bare}")).await;
        assert_eq!((status, cache), (StatusCode::OK, None));
    }
    let (status, _) = cache_of(&app, "/illustrations/grand/nothing.jpg").await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

fn setup(seed: u64) -> Setup {
    Setup {
        seed,
        players: 3,
        investigators: vec![InvestigatorId::new("amanda_sharpe")],
        ancient_one: None,
    }
}

fn load_data() -> GameData {
    GameData::load(Path::new("../../data")).expect("data")
}

/// A new game, answered with the first option until `stop` holds.
async fn game_until(
    db: &Database,
    data: &GameData,
    seed: u64,
    stop: impl Fn(&Session) -> bool,
) -> Session {
    let mut session = Session::start(db, data, setup(seed), PlayMode::Solo)
        .await
        .expect("start");
    for _ in 0..500 {
        if stop(&session) {
            return session;
        }
        let number = session.move_number();
        session.answer(data, 0, number).expect("answer");
    }
    panic!("never reached");
}

/// The part of a page between `start` and the next `end`.
fn between<'p>(page: &'p str, start: &str, end: &str) -> &'p str {
    let from = page.find(start).expect(start);
    let rest = &page[from..];
    &rest[..rest.find(end).expect(end)]
}

#[tokio::test(flavor = "current_thread")]
async fn the_map_offers_every_proposed_destination() {
    let dir = temp();
    let db = Database::open(&dir.path().join("db")).await.expect("db");
    let data = load_data();
    let moving = |session: &Session| {
        session
            .decision()
            .is_some_and(|d| d.kind == DecisionKind::Move && !session.in_combat())
    };
    let session = game_until(&db, &data, 5, moving).await;
    let decision = session.decision().expect("decision");
    let places: Vec<usize> = decision
        .options
        .iter()
        .enumerate()
        .filter(|(_, option)| matches!(option.choice, Choice::Place(_) | Choice::Gate(_)))
        .map(|(index, _)| index)
        .collect();
    assert!(!places.is_empty());
    let page = game_page(&data, &session, "../".to_owned(), false).expect("page");
    let map = between(&page, "<section class=\"map\"", "</section>");
    assert!(map.contains("<svg"));
    for index in &places {
        let target = format!(
            "form=\"decision\" name=\"choix\" value=\"{index}\" class=\"map-target offered"
        );
        assert!(map.contains(&target), "no target for option {index}");
    }
    assert_eq!(
        map.matches("class=\"map-target offered").count(),
        places.len()
    );
    assert!(
        map.contains("class=\"space location offered")
            || map.contains("class=\"space street offered")
    );
    assert!(map.contains("href=\"partie/1?voir="), "preview links");
    db.close().await;
}
