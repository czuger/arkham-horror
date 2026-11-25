use axum::http::StatusCode;

use crate::tests::test_app::{TestApp, move_number};

const NEW_GAME: &str = "investigateur=amanda_sharpe&ia=2&ia1=&ia2=&grand_ancien=&graine=7";
const HOT_SEAT: &str = "investigateur=amanda_sharpe&ia=1&ia1=bob_jenkins&graine=7&tous=1";
const GAME: &str = "/partie/1";
const ROSTER: &str = "/partie/1/ajustement";

/// One investigator's adjustment page.
fn adjustment(seat: usize) -> String {
    format!("{ROSTER}/{seat}")
}

/// The page where the current decision is answered.
async fn current_page(app: &TestApp) -> String {
    let page = app.get(GAME).await.body;
    if page.contains("class=\"on-sheet\"") {
        app.get(&format!("{GAME}/investigateurs")).await.body
    } else if page.contains("class=\"in-combat") {
        app.get(&format!("{GAME}/combat")).await.body
    } else {
        page
    }
}

/// Creates a game with `form` and plays first options until the game page
/// sends to the adjustment screen.
async fn play_to_upkeep(app: &TestApp, form: &str) -> String {
    app.post("/nouvelle", form).await;
    for _ in 0..300 {
        let game = app.get(GAME).await.body;
        if game.contains("href=\"partie/1/ajustement\"") {
            return game;
        }
        let page = current_page(app).await;
        let coup = move_number(&page).expect("a decision");
        app.post(&format!("{GAME}/reponse"), &format!("choix=0&coup={coup}"))
            .await;
    }
    panic!("no Upkeep adjustment reached");
}

/// Stops one move away from `before` on the first track and two on the
/// second: three stops in all.
fn three_stops_from(before: [u8; 3]) -> [u8; 3] {
    let mut wanted = before;
    wanted[0] = if before[0] < 3 {
        before[0] + 1
    } else {
        before[0] - 1
    };
    wanted[1] = if before[1] < 2 {
        before[1] + 2
    } else {
        before[1] - 2
    };
    wanted
}

/// One stop away from `before`, on the first track.
fn one_stop_from(before: [u8; 3]) -> [u8; 3] {
    let mut wanted = before;
    wanted[0] = if before[0] < 3 {
        before[0] + 1
    } else {
        before[0] - 1
    };
    wanted
}

/// Whether the sheet of `seat` shows its sliders where `page`, an
/// adjustment page of that seat, puts the stops `wanted`.
async fn sliders_on(app: &TestApp, seat: usize, page: &str, wanted: [u8; 3]) -> bool {
    let sheet = app.get(&format!("{GAME}/investigateurs/{seat}")).await.body;
    wanted.iter().enumerate().all(|(track, stop)| {
        let style = stop_style(page, track, *stop);
        sheet.contains(&format!("alt=\"Curseur\" style=\"{style}\""))
    })
}

/// The stop each slider stands on, read from the adjustment page.
fn current_stops(page: &str) -> [u8; 3] {
    [0, 1, 2].map(|track| {
        let marker = format!("name=\"piste{track}\" value=\"0\" data-current=\"");
        let start = page.find(&marker).expect("track") + marker.len();
        page[start..start + 1].parse().expect("stop")
    })
}

/// `left: …; top: …` of a stop, read from the adjustment page.
fn stop_style(page: &str, track: usize, stop: u8) -> String {
    let input = page
        .find(&format!("name=\"piste{track}\" value=\"{stop}\""))
        .expect("stop");
    let start = page[..input].rfind("style=\"").expect("style") + "style=\"".len();
    let end = start + page[start..].find('"').expect("end");
    page[start..end].to_owned()
}

fn form(coup: usize, stops: [u8; 3]) -> String {
    format!(
        "coup={coup}&piste0={}&piste1={}&piste2={}",
        stops[0], stops[1], stops[2]
    )
}

#[tokio::test(flavor = "current_thread")]
async fn the_adjustment_screen_lists_everyone_and_plays_the_picks_on_moving_on() {
    let dir = tempfile::tempdir().expect("temp dir");
    let app = TestApp::open(&dir.path().join("db")).await;
    let game = play_to_upkeep(&app, NEW_GAME).await;
    assert!(!game.contains("class=\"on-sheet\""), "{game}");
    let reply = app.post(&format!("{GAME}/reponse"), "choix=0&coup=0").await;
    assert_eq!(reply.status, StatusCode::SEE_OTHER, "stale answer ignored");

    let roster = app.get(ROSTER).await;
    assert_eq!(roster.status, StatusCode::OK);
    assert_eq!(roster.body.matches("<li class=\"adjuster").count(), 3);
    assert_eq!(
        roster.body.matches("Ajuster les compétences</a>").count(),
        1
    );
    assert!(roster.body.contains("href=\"partie/1/ajustement/0\""));
    assert!(roster.body.contains(">Passer</button>"));
    assert!(roster.body.contains("Curseurs inchangés."));
    let coup = move_number(&roster.body).expect("a decision");

    let page = app.get(&adjustment(0)).await;
    assert_eq!(page.status, StatusCode::OK);
    assert_eq!(page.body.matches("class=\"stop-choice").count(), 12);
    assert_eq!(page.body.matches(" checked").count(), 3);
    assert!(page.body.contains("action=\"partie/1/ajustement/0\""));
    let before = current_stops(&page.body);
    // An AI seat is not adjusted by hand.
    let ai = app.get(&adjustment(1)).await;
    assert_eq!(ai.location.as_deref(), Some(ROSTER));

    // Out of reach or unknown: refused, nothing kept.
    let farthest = before.map(|stop| if stop < 2 { 3 } else { 0 });
    let refused = app.post(&adjustment(0), &form(coup, farthest)).await;
    assert_eq!(refused.status, StatusCode::BAD_REQUEST);
    let unknown = app.post(&adjustment(0), &form(coup, [9, 9, 9])).await;
    assert_eq!(unknown.status, StatusCode::BAD_REQUEST);
    assert_eq!(app.get(ROSTER).await.body, roster.body);

    // Picked: kept aside, shown, not played yet.
    let wanted = three_stops_from(before);
    let reply = app.post(&adjustment(0), &form(coup, wanted)).await;
    assert_eq!(reply.status, StatusCode::SEE_OTHER, "{}", reply.body);
    assert_eq!(reply.location.as_deref(), Some(ROSTER));
    assert!(app.get(ROSTER).await.body.contains("Curseurs déplacés."));
    let picked = app.get(&adjustment(0)).await.body;
    assert_eq!(current_stops(&picked), before, "not played yet");
    assert!(!sliders_on(&app, 0, &page.body, wanted).await);

    // Moving on plays it.
    let reply = app.post(ROSTER, &format!("coup={coup}")).await;
    assert_eq!(reply.status, StatusCode::SEE_OTHER, "{}", reply.body);
    assert_eq!(app.get(ROSTER).await.status, StatusCode::SEE_OTHER, "over");
    assert!(sliders_on(&app, 0, &page.body, wanted).await);

    // One undo takes the whole adjustment back, picks forgotten.
    app.post(&format!("{GAME}/annuler"), "").await;
    assert_eq!(app.get(ROSTER).await.body, roster.body);

    // Moving on without a pick keeps the sliders.
    let skip = app.post(ROSTER, &format!("coup={coup}")).await;
    assert_eq!(skip.status, StatusCode::SEE_OTHER);
    assert_eq!(app.get(ROSTER).await.status, StatusCode::SEE_OTHER);
    assert!(sliders_on(&app, 0, &page.body, before).await);
}

#[tokio::test(flavor = "current_thread")]
async fn in_a_hot_seat_game_every_investigator_is_adjusted_in_any_order() {
    let dir = tempfile::tempdir().expect("temp dir");
    let app = TestApp::open(&dir.path().join("db")).await;
    play_to_upkeep(&app, HOT_SEAT).await;
    let roster = app.get(ROSTER).await.body;
    assert_eq!(roster.matches("Ajuster les compétences</a>").count(), 2);
    let coup = move_number(&roster).expect("a decision");

    let mut pages = Vec::new();
    for seat in [1, 0] {
        let page = app.get(&adjustment(seat)).await.body;
        let wanted = one_stop_from(current_stops(&page));
        let reply = app.post(&adjustment(seat), &form(coup, wanted)).await;
        assert_eq!(reply.status, StatusCode::SEE_OTHER, "{}", reply.body);
        pages.push((seat, page, wanted));
    }
    assert_eq!(
        app.get(ROSTER)
            .await
            .body
            .matches("Curseurs déplacés.")
            .count(),
        2
    );
    app.post(ROSTER, &format!("coup={coup}")).await;
    assert_eq!(app.get(ROSTER).await.status, StatusCode::SEE_OTHER, "over");
    for (seat, page, wanted) in pages {
        assert!(sliders_on(&app, seat, &page, wanted).await, "seat {seat}");
    }
}
