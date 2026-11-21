use axum::body::Body;
use axum::http::{Request, StatusCode, header};

use crate::tests::test_app::{TestApp, move_number};

const NEW_GAME: &str = "investigateur=amanda_sharpe&ia=2&ia1=&ia2=&grand_ancien=&graine=7";

fn temp() -> tempfile::TempDir {
    tempfile::tempdir().expect("temp dir")
}

/// Creates a game and returns its page path.
async fn new_game(app: &TestApp, form: &str) -> String {
    let reply = app.post("/nouvelle", form).await;
    assert_eq!(reply.status, StatusCode::SEE_OTHER, "{}", reply.body);
    reply.location.expect("redirect")
}

/// Answers the first option of the current decision.
async fn answer_first(app: &TestApp, game: &str) -> String {
    let page = app.get(game).await.body;
    let coup = move_number(&page).expect("a decision");
    let reply = app
        .post(&format!("{game}/reponse"), &format!("choix=0&coup={coup}"))
        .await;
    assert_eq!(reply.status, StatusCode::SEE_OTHER, "{}", reply.body);
    app.get(game).await.body
}

#[tokio::test(flavor = "current_thread")]
async fn home_page_lists_no_game_at_first() {
    let dir = temp();
    let app = TestApp::open(&dir.path().join("db"));
    let reply = app.get("/").await;
    assert_eq!(reply.status, StatusCode::OK);
    assert!(reply.body.contains("Aucune partie"));
    assert_eq!(reply.robots.as_deref(), Some("noindex, nofollow"));
}

#[tokio::test(flavor = "current_thread")]
async fn robots_txt_and_static_files_are_served() {
    let dir = temp();
    let app = TestApp::open(&dir.path().join("db"));
    let robots = app.get("/robots.txt").await;
    assert!(robots.body.contains("Disallow: /"));
    assert_eq!(app.get("/static/style.css").await.status, StatusCode::OK);
    assert_eq!(app.get("/nouvelle").await.status, StatusCode::OK);
}

#[tokio::test(flavor = "current_thread")]
async fn a_new_game_waits_for_the_human() {
    let dir = temp();
    let app = TestApp::open(&dir.path().join("db"));
    let game = new_game(&app, NEW_GAME).await;
    assert_eq!(game, "/partie/1");
    let page = app.get(&game).await;
    assert_eq!(page.status, StatusCode::OK);
    assert!(
        page.body.contains("Amanda Sharpe : à vous"),
        "{}",
        page.body
    );
    assert!(app.get("/").await.body.contains("Amanda Sharpe et 2 IA"));
}

#[tokio::test(flavor = "current_thread")]
async fn the_ancient_one_page_shows_its_sheet() {
    let dir = temp();
    let app = TestApp::open(&dir.path().join("db"));
    let game = new_game(&app, NEW_GAME).await;
    assert!(app.get(&game).await.body.contains("/grand-ancien\""));
    let page = app.get(&format!("{game}/grand-ancien")).await;
    assert_eq!(page.status, StatusCode::OK);
    assert!(
        page.body.contains("fiches/grands_anciens/"),
        "{}",
        page.body
    );
    assert_eq!(
        app.get("/partie/99/grand-ancien").await.status,
        StatusCode::NOT_FOUND
    );
}

#[tokio::test(flavor = "current_thread")]
async fn the_investigators_page_shows_every_sheet_with_its_tokens() {
    let dir = temp();
    let app = TestApp::open(&dir.path().join("db"));
    let game = new_game(&app, NEW_GAME).await;
    assert!(app.get(&game).await.body.contains("/investigateurs\""));
    let page = app.get(&format!("{game}/investigateurs")).await;
    assert_eq!(page.status, StatusCode::OK);
    let body = page.body;
    assert_eq!(body.matches("class=\"investigator-sheet\"").count(), 3);
    assert_eq!(body.matches("class=\"slider\"").count(), 9);
    assert!(body.contains("marqueurs/san.jpg"), "{body}");
    assert!(body.contains("marqueurs/res.jpg"), "{body}");
    assert!(body.contains("Trophées"), "{body}");
    assert_eq!(
        app.get("/partie/99/investigateurs").await.status,
        StatusCode::NOT_FOUND
    );
}

#[tokio::test(flavor = "current_thread")]
async fn the_landscape_board_page_shows_the_turned_board() {
    let dir = temp();
    let app = TestApp::open(&dir.path().join("db"));
    let game = new_game(&app, NEW_GAME).await;
    assert!(app.get(&game).await.body.contains("/plateau\""));
    let page = app.get(&format!("{game}/plateau")).await;
    assert_eq!(page.status, StatusCode::OK);
    assert!(
        page.body.contains("width: 1576px; height: 1064px"),
        "{}",
        page.body
    );
    assert!(page.body.contains("rotate(90deg)"), "{}", page.body);
    assert!(!page.body.contains("<button"), "{}", page.body);
    assert_eq!(
        app.get("/partie/99/plateau").await.status,
        StatusCode::NOT_FOUND
    );
}

#[tokio::test(flavor = "current_thread")]
async fn locations_show_their_text_and_icons_on_hover() {
    let dir = temp();
    let app = TestApp::open(&dir.path().join("db"));
    let game = new_game(&app, NEW_GAME).await;
    let page = app.get(&game).await.body;
    assert_eq!(page.matches("class=\"panel tip").count(), 26);
    assert!(
        page.contains("<b>Prêt Bancaire</b> : Au lieu de faire"),
        "{page}"
    );
    assert!(page.contains("<b>Objet Unique</b> : on y trouve souvent"));
    assert!(page.contains("Couleurs inversées : moyen sûr"));
}

#[tokio::test(flavor = "current_thread")]
async fn answers_move_the_game_on_and_can_be_undone() {
    let dir = temp();
    let app = TestApp::open(&dir.path().join("db"));
    let game = new_game(&app, NEW_GAME).await;
    let first = app.get(&game).await.body;
    let after_one = answer_first(&app, &game).await;
    assert!(move_number(&after_one) > move_number(&first));
    let reply = app.post(&format!("{game}/annuler"), "").await;
    assert_eq!(reply.status, StatusCode::SEE_OTHER);
    assert_eq!(app.get(&game).await.body, first);
}

#[tokio::test(flavor = "current_thread")]
async fn bad_and_stale_answers_are_refused() {
    let dir = temp();
    let app = TestApp::open(&dir.path().join("db"));
    let game = new_game(&app, NEW_GAME).await;
    let page = app.get(&game).await.body;
    let coup = move_number(&page).expect("decision");
    let bad = app
        .post(
            &format!("{game}/reponse"),
            &format!("choix=999&coup={coup}"),
        )
        .await;
    assert_eq!(bad.status, StatusCode::BAD_REQUEST);
    let stale = app
        .post(
            &format!("{game}/reponse"),
            &format!("choix=0&coup={}", coup + 5),
        )
        .await;
    assert_eq!(stale.status, StatusCode::SEE_OTHER);
    assert_eq!(app.get(&game).await.body, page);
    assert_eq!(app.get("/partie/99").await.status, StatusCode::NOT_FOUND);
}

#[tokio::test(flavor = "current_thread")]
async fn bad_setups_are_shown_again() {
    let dir = temp();
    let app = TestApp::open(&dir.path().join("db"));
    let twice = "investigateur=amanda_sharpe&ia=1&ia1=amanda_sharpe&graine=";
    let reply = app.post("/nouvelle", twice).await;
    assert_eq!(reply.status, StatusCode::BAD_REQUEST);
    assert!(reply.body.contains("choisi deux fois"));
}

#[tokio::test(flavor = "current_thread")]
async fn a_game_resumes_after_a_restart() {
    let dir = temp();
    let path = dir.path().join("db");
    let app = TestApp::open(&path);
    let game = new_game(&app, NEW_GAME).await;
    for _ in 0..5 {
        answer_first(&app, &game).await;
    }
    let before = app.get(&game).await.body;
    drop(app);
    let app = TestApp::open(&path);
    assert_eq!(app.get(&game).await.body, before);
}

#[tokio::test(flavor = "current_thread")]
async fn cross_site_forms_are_refused() {
    let dir = temp();
    let app = TestApp::open(&dir.path().join("db"));
    let request = Request::post("/nouvelle")
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .header("sec-fetch-site", "cross-site")
        .body(Body::from(NEW_GAME))
        .expect("request");
    assert_eq!(app.send(request).await.status, StatusCode::FORBIDDEN);
    assert!(app.get("/").await.body.contains("Aucune partie"));
}

/// A whole game with two AIs, played through the forms by always picking
/// the first option, ends with a victory or a defeat.
#[tokio::test(flavor = "current_thread")]
async fn a_whole_game_is_played_through_the_forms() {
    let dir = temp();
    let app = TestApp::open(&dir.path().join("db"));
    let game = new_game(&app, "investigateur=amanda_sharpe&ia=2&graine=3").await;
    let mut page = app.get(&game).await.body;
    let mut answers = 0;
    while move_number(&page).is_some() {
        page = answer_first(&app, &game).await;
        answers += 1;
        assert!(answers < 5_000, "the game never ends");
    }
    assert!(
        page.contains("Victoire") || page.contains("Défaite"),
        "{page}"
    );
    assert!(app.get("/").await.body.contains("class=\"over\""));
    println!("{answers} human answers");
}
