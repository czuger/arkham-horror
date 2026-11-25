use axum::body::Body;
use axum::http::{Request, StatusCode, header};

use tower::ServiceExt;

use crate::tests::test_app::{TestApp, move_number};

const NEW_GAME: &str = "investigateur=amanda_sharpe&ia=2&ia1=&ia2=&grand_ancien=&graine=7";

fn temp() -> tempfile::TempDir {
    tempfile::tempdir().expect("temp dir")
}

/// Creates a game and returns its page path.
async fn new_game(app: &TestApp, form: &str) -> String {
    let reply = app.post("/nouvelle", form).await;
    assert_eq!(reply.status, StatusCode::SEE_OTHER, "{}", reply.body);
    let location = reply.location.expect("redirect");
    location.trim_end_matches(SHEET).to_owned()
}

/// Path of the human investigator's page, after the game's.
const SHEET: &str = "/investigateurs";

/// The page where the current decision is answered: the game page, or the
/// human investigator's page when the game page sends there.
async fn current_page(app: &TestApp, game: &str) -> String {
    let page = app.get(game).await.body;
    if page.contains("class=\"on-sheet\"") {
        app.get(&format!("{game}{SHEET}")).await.body
    } else if page.contains("class=\"in-combat") {
        app.get(&format!("{game}/combat")).await.body
    } else if page.contains("class=\"adjust-skills") {
        app.get(&format!("{game}/ajustement")).await.body
    } else {
        page
    }
}

/// Answers the first option of the current decision.
async fn answer_first(app: &TestApp, game: &str) -> String {
    let page = current_page(app, game).await;
    let coup = move_number(&page).expect("a decision");
    let reply = app
        .post(&format!("{game}/reponse"), &format!("choix=0&coup={coup}"))
        .await;
    assert_eq!(reply.status, StatusCode::SEE_OTHER, "{}", reply.body);
    current_page(app, game).await
}

#[tokio::test(flavor = "current_thread")]
async fn home_page_lists_no_game_at_first() {
    let dir = temp();
    let app = TestApp::open(&dir.path().join("db")).await;
    let reply = app.get("/").await;
    assert_eq!(reply.status, StatusCode::OK);
    assert!(reply.body.contains("Aucune partie"));
    assert_eq!(reply.robots.as_deref(), Some("noindex, nofollow"));
}

#[tokio::test(flavor = "current_thread")]
async fn robots_txt_and_static_files_are_served() {
    let dir = temp();
    let app = TestApp::open(&dir.path().join("db")).await;
    let robots = app.get("/robots.txt").await;
    assert!(robots.body.contains("Disallow: /"));
    assert_eq!(app.get("/static/style.css").await.status, StatusCode::OK);
    assert_eq!(app.get("/nouvelle").await.status, StatusCode::OK);
}

#[tokio::test(flavor = "current_thread")]
async fn built_in_images_are_served_with_their_type() {
    let dir = temp();
    let app = TestApp::open(&dir.path().join("db")).await;
    let game = new_game(&app, NEW_GAME).await;
    let old_board = app.get(&format!("{game}/plateau")).await;
    assert_eq!(
        old_board.status,
        StatusCode::NOT_FOUND,
        "page removed in J6"
    );
    let body = app.get(&format!("{game}{SHEET}")).await.body;
    let start = body.find("src=\"assets/").expect("an image") + "src=\"".len();
    let image = &body[start..start + body[start..].find('"').expect("end")];
    let request = Request::get(format!("/{image}"))
        .body(Body::empty())
        .expect("request");
    let response = app.router.clone().oneshot(request).await.expect("response");
    assert_eq!(response.status(), StatusCode::OK, "{image}");
    let kind = response.headers().get(header::CONTENT_TYPE).expect("type");
    assert!(
        kind.to_str().expect("text").starts_with("image/"),
        "{kind:?}"
    );
    assert_eq!(
        app.get("/assets/nothing.png").await.status,
        StatusCode::NOT_FOUND
    );
}

/// Every page links relatively, from the root its `<base>` gives back: the
/// application may be served under a sub-path.
#[tokio::test(flavor = "current_thread")]
async fn pages_link_relatively() {
    let dir = temp();
    let app = TestApp::open(&dir.path().join("db")).await;
    let game = new_game(&app, NEW_GAME).await;
    let pages = [
        ("/".to_owned(), "./"),
        ("/nouvelle".to_owned(), "./"),
        (game.clone(), "../"),
        (format!("{game}/journal"), "../../"),
        (format!("{game}/combat"), "../../"),
        (format!("{game}/grand-ancien"), "../../"),
        (format!("{game}{SHEET}"), "../../"),
        (format!("{game}{SHEET}/1"), "../../../"),
    ];
    for (path, root) in pages {
        let body = app.get(&path).await.body;
        assert!(body.contains(&format!("<base href=\"{root}\">")), "{path}");
        for absolute in ["href=\"/", "src=\"/", "action=\"/", "url('/"] {
            assert!(!body.contains(absolute), "{path} contains {absolute}");
        }
    }
}

#[tokio::test(flavor = "current_thread")]
async fn a_new_game_waits_for_the_human() {
    let dir = temp();
    let app = TestApp::open(&dir.path().join("db")).await;
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
async fn a_hot_seat_game_lets_the_human_play_every_investigator() {
    let dir = temp();
    let app = TestApp::open(&dir.path().join("db")).await;
    let form = "investigateur=amanda_sharpe&ia=1&ia1=bob_jenkins&graine=7&tous=1";
    let game = new_game(&app, form).await;
    assert!(
        app.get("/")
            .await
            .body
            .contains("Amanda Sharpe et Bob Jenkins contre"),
    );
    let other = app.get(&format!("{game}/investigateurs/1")).await.body;
    assert!(other.contains("(Vous ·"), "{other}");
    let mut page = current_page(&app, &game).await;
    for _ in 0..40 {
        if page.contains("Bob Jenkins : à vous") {
            return;
        }
        page = answer_first(&app, &game).await;
    }
    panic!("Bob Jenkins never had a decision: {page}");
}

#[tokio::test(flavor = "current_thread")]
async fn the_ancient_one_page_shows_its_sheet() {
    let dir = temp();
    let app = TestApp::open(&dir.path().join("db")).await;
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
async fn each_investigator_has_a_page_with_tokens_and_possessions() {
    let dir = temp();
    let app = TestApp::open(&dir.path().join("db")).await;
    let game = new_game(&app, NEW_GAME).await;
    assert!(app.get(&game).await.body.contains("/investigateurs\""));
    let own = app.get(&format!("{game}/investigateurs")).await;
    assert_eq!(own.status, StatusCode::OK);
    let body = own.body;
    assert_eq!(body.matches("class=\"investigator-sheet\"").count(), 1);
    assert_eq!(body.matches("class=\"slider\"").count(), 3);
    assert!(body.contains("<h1>Amanda Sharpe"), "{body}");
    assert!(body.contains("marqueurs/san.png"), "{body}");
    assert!(body.contains("marqueurs/res.png"), "{body}");
    assert!(body.contains("marqueurs/dollar_1.png"), "{body}");
    assert!(body.contains("Trophées"), "{body}");
    assert!(body.contains("class=\"held-card"), "{body}");
    assert!(body.contains("En main : 0 / 2 mains"), "{body}");
    for seat in 0..3 {
        assert!(
            body.contains(&format!("/investigateurs/{seat}\"")),
            "{body}"
        );
    }
    let other = app.get(&format!("{game}/investigateurs/1")).await;
    assert_eq!(other.status, StatusCode::OK);
    assert!(other.body.contains("(IA ·"), "{}", other.body);
    assert!(!other.body.contains("name=\"choix\""), "{}", other.body);
    for missing in [
        "/partie/99/investigateurs",
        &format!("{game}/investigateurs/3"),
    ] {
        assert_eq!(app.get(missing).await.status, StatusCode::NOT_FOUND);
    }
}

#[tokio::test(flavor = "current_thread")]
async fn sliders_are_placed_on_the_sheet() {
    let dir = temp();
    let app = TestApp::open(&dir.path().join("db")).await;
    let mut next = app.post("/nouvelle", NEW_GAME).await.location;
    // Starting cards may be picked first, on the game page.
    for _ in 0..10 {
        if next.as_deref() != Some("/partie/1") {
            break;
        }
        let coup = move_number(&app.get("/partie/1").await.body).expect("a decision");
        next = app
            .post("/partie/1/reponse", &format!("choix=0&coup={coup}"))
            .await
            .location;
    }
    assert_eq!(next.as_deref(), Some("/partie/1/investigateurs"));
    let game_page = app.get("/partie/1").await.body;
    assert!(game_page.contains("class=\"on-sheet\""), "{game_page}");
    assert!(!game_page.contains("name=\"choix\""), "{game_page}");
    let mut sheet = app.get("/partie/1/investigateurs").await.body;
    for _ in 0..3 {
        assert_eq!(sheet.matches("class=\"stop-target\"").count(), 4, "{sheet}");
        let coup = move_number(&sheet).expect("a decision");
        let reply = app
            .post("/partie/1/reponse", &format!("choix=3&coup={coup}"))
            .await;
        assert_eq!(reply.status, StatusCode::SEE_OTHER);
        let next = reply.location.expect("redirect");
        if next == "/partie/1" {
            break;
        }
        sheet = app.get(&next).await.body;
    }
    let sheet = app.get("/partie/1/investigateurs").await.body;
    assert!(!sheet.contains("stop-target"), "{sheet}");
}

#[tokio::test(flavor = "current_thread")]
async fn locations_show_their_text_and_icons_on_hover() {
    let dir = temp();
    let app = TestApp::open(&dir.path().join("db")).await;
    let game = new_game(&app, NEW_GAME).await;
    let page = app.get(&game).await.body;
    let map = &page[page.find("class=\"map-targets\"").expect("map")..];
    assert!(map.matches("class=\"bubble place-bubble\"").count() >= 26);
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
    let app = TestApp::open(&dir.path().join("db")).await;
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
    let app = TestApp::open(&dir.path().join("db")).await;
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
    let app = TestApp::open(&dir.path().join("db")).await;
    let twice = "investigateur=amanda_sharpe&ia=1&ia1=amanda_sharpe&graine=";
    let reply = app.post("/nouvelle", twice).await;
    assert_eq!(reply.status, StatusCode::BAD_REQUEST);
    assert!(reply.body.contains("choisi deux fois"));
}

#[tokio::test(flavor = "current_thread")]
async fn a_game_resumes_after_a_restart() {
    let dir = temp();
    let path = dir.path().join("db");
    let app = TestApp::open(&path).await;
    let game = new_game(&app, NEW_GAME).await;
    for _ in 0..5 {
        answer_first(&app, &game).await;
    }
    let before = app.get(&game).await.body;
    drop(app);
    let app = TestApp::open(&path).await;
    assert_eq!(app.get(&game).await.body, before);
}

#[tokio::test(flavor = "current_thread")]
async fn cross_site_forms_are_refused() {
    let dir = temp();
    let app = TestApp::open(&dir.path().join("db")).await;
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
    let app = TestApp::open(&dir.path().join("db")).await;
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

/// Cards drawn show in the journal; a monster faced sends to the combat
/// page, which shows the monster and every die rolled.
#[tokio::test(flavor = "current_thread")]
async fn drawn_cards_and_combat_dice_are_shown() {
    let dir = temp();
    let app = TestApp::open(&dir.path().join("db")).await;
    let mut cards_seen = false;
    let mut combat_seen = false;
    for seed in 1..20 {
        let form = format!("investigateur=amanda_sharpe&ia=0&graine={seed}");
        let game = new_game(&app, &form).await;
        for _ in 0..300 {
            let page = current_page(&app, &game).await;
            let Some(coup) = move_number(&page) else {
                break;
            };
            let reply = app
                .post(&format!("{game}/reponse"), &format!("choix=0&coup={coup}"))
                .await;
            if reply.location.as_deref() == Some(&format!("{game}/combat")) {
                let combat = app.get(&format!("{game}/combat")).await.body;
                assert!(combat.contains("class=\"foe\""), "{combat}");
                assert!(
                    combat.contains("class=\"scene-image\" src=\"illustrations/grand/monstres/"),
                    "the monster's portrait is the scene"
                );
                assert!(combat.contains("<section class=\"map\""), "the map stays");
                combat_seen |= combat.contains("class=\"die");
            }
            cards_seen |= app
                .get(&game)
                .await
                .body
                .contains("<figcaption>Carte Mythe tirée par <strong>");
            if cards_seen && combat_seen {
                return;
            }
        }
    }
    assert!(cards_seen, "no Mythos card shown");
    assert!(combat_seen, "no combat with dice shown");
}
