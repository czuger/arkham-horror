use axum::http::StatusCode;

use crate::tests::test_app::{TestApp, move_number};

const NEW_GAME: &str = "investigateur=amanda_sharpe&ia=2&ia1=&ia2=&grand_ancien=&graine=7";
const GAME: &str = "/partie/1";
const ADJUSTMENT: &str = "/partie/1/ajustement";

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

/// Plays first options until the game page offers the skill adjustment.
async fn play_to_upkeep(app: &TestApp) -> String {
    app.post("/nouvelle", NEW_GAME).await;
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
async fn the_upkeep_adjustment_can_be_skipped_or_set_in_one_form() {
    let dir = tempfile::tempdir().expect("temp dir");
    let app = TestApp::open(&dir.path().join("db")).await;
    let game = play_to_upkeep(&app).await;
    assert!(game.contains("Passer : ne rien changer"), "{game}");
    assert!(!game.contains("class=\"on-sheet\""), "{game}");

    let page = app.get(ADJUSTMENT).await;
    assert_eq!(page.status, StatusCode::OK);
    assert_eq!(page.body.matches("class=\"stop-choice").count(), 12);
    assert_eq!(page.body.matches(" checked").count(), 3);
    let coup = move_number(&page.body).expect("a decision");
    let before = current_stops(&page.body);

    // Out of reach: refused, nothing saved.
    let farthest = before.map(|stop| if stop < 2 { 3 } else { 0 });
    let refused = app.post(ADJUSTMENT, &form(coup, farthest)).await;
    assert_eq!(refused.status, StatusCode::BAD_REQUEST);
    let unknown = app.post(ADJUSTMENT, &form(coup, [9, 9, 9])).await;
    assert_eq!(unknown.status, StatusCode::BAD_REQUEST);
    assert_eq!(app.get(ADJUSTMENT).await.body, page.body);

    // Two stops on the first track and one on the second, in one post.
    let mut wanted = before;
    wanted[0] = if before[0] < 2 {
        before[0] + 2
    } else {
        before[0] - 2
    };
    wanted[1] = if before[1] < 3 {
        before[1] + 1
    } else {
        before[1] - 1
    };
    let reply = app.post(ADJUSTMENT, &form(coup, wanted)).await;
    assert_eq!(reply.status, StatusCode::SEE_OTHER, "{}", reply.body);
    let moved = app.get(ADJUSTMENT).await;
    assert_eq!(moved.status, StatusCode::SEE_OTHER, "adjustment over");
    let sheet = app.get(&format!("{GAME}/investigateurs/0")).await.body;
    for (track, stop) in wanted.iter().enumerate() {
        let style = stop_style(&page.body, track, *stop);
        assert!(
            sheet.contains(&format!("alt=\"Curseur\" style=\"{style}\"")),
            "{style}"
        );
    }

    // One undo takes the whole adjustment back.
    app.post(&format!("{GAME}/annuler"), "").await;
    assert_eq!(app.get(ADJUSTMENT).await.body, page.body);

    // Skipping keeps the sliders.
    let skip = app
        .post(&format!("{GAME}/reponse"), &format!("choix=0&coup={coup}"))
        .await;
    assert_eq!(skip.status, StatusCode::SEE_OTHER);
    assert_eq!(app.get(ADJUSTMENT).await.status, StatusCode::SEE_OTHER);
}
