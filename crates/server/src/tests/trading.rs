//! Trading on its own page: the AI partner always accepts.

use axum::http::StatusCode;

use crate::tests::test_app::{TestApp, move_number};

const NEW_GAME: &str = "investigateur=amanda_sharpe&ia=2&ia1=&ia2=&grand_ancien=&graine=5";
const GAME: &str = "/partie/1";
const TRADE: &str = "/partie/1/echange";

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

/// The answer index of the button labelled `label`.
fn button(page: &str, label: &str) -> Option<usize> {
    let end = page.find(&format!("\">{label}"))?;
    let start = page[..end].rfind("value=\"")? + "value=\"".len();
    page[start..end].parse().ok()
}

/// Plays first options until the game page offers a trade; returns the page.
async fn play_to_a_trade(app: &TestApp) -> String {
    app.post("/nouvelle", NEW_GAME).await;
    for _ in 0..300 {
        let page = current_page(app).await;
        if page.contains("Échanger avec") {
            return page;
        }
        let coup = move_number(&page).expect("a decision");
        app.post(&format!("{GAME}/reponse"), &format!("choix=0&coup={coup}"))
            .await;
    }
    panic!("no trade offered");
}

/// The value of `max` of the number field `name`.
fn most(page: &str, name: &str) -> u16 {
    let field = page.find(&format!("name=\"{name}\"")).expect(name);
    let rest = &page[field..];
    let start = rest.find("max=\"").expect("max") + "max=\"".len();
    let end = start + rest[start..].find('"').expect("end");
    rest[start..end].parse().expect("amount")
}

#[tokio::test(flavor = "current_thread")]
async fn a_trade_is_picked_on_its_page_and_the_ai_accepts_it() {
    let dir = tempfile::tempdir().expect("temp dir");
    let app = TestApp::open(&dir.path().join("db")).await;
    let game = play_to_a_trade(&app).await;
    let coup = move_number(&game).expect("a decision");
    let index = button(&game, "Échanger avec").expect("trade button");

    // Picking the partner leads straight to the trade page.
    let reply = app
        .post(
            &format!("{GAME}/reponse"),
            &format!("choix={index}&coup={coup}"),
        )
        .await;
    assert_eq!(reply.location.as_deref(), Some(TRADE), "{}", reply.body);
    let page = app.get(TRADE).await;
    assert_eq!(page.status, StatusCode::OK);
    assert!(page.body.contains("Proposer l'échange"), "{}", page.body);
    assert!(page.body.contains("accepte toujours"));
    let money = most(&page.body, "argent_donne");
    let partner_money = most(&page.body, "argent_pris");
    let coup = move_number(&page.body).expect("a decision");

    // More money than the partner has: refused, nothing saved.
    let too_much = format!(
        "coup={coup}&action=proposer&argent_pris={}",
        partner_money + 1
    );
    assert_eq!(
        app.post(TRADE, &too_much).await.status,
        StatusCode::BAD_REQUEST
    );
    assert_eq!(app.get(TRADE).await.body, page.body);

    // One dollar one way or the other: the AI accepts, back to the movement.
    let (field, after) = if money > 0 {
        ("argent_donne", (money - 1, partner_money + 1))
    } else {
        assert!(partner_money > 0, "someone has money");
        ("argent_pris", (money + 1, partner_money - 1))
    };
    let reply = app
        .post(TRADE, &format!("coup={coup}&action=proposer&{field}=1"))
        .await;
    assert_eq!(reply.status, StatusCode::SEE_OTHER);
    let journal = app.get(&format!("{GAME}/journal")).await.body;
    assert!(journal.contains("donne à"), "{journal}");
    assert!(journal.contains(": 1 $."), "{journal}");
    let game = current_page(&app).await;
    assert!(game.contains("Terminer le mouvement ici"), "{game}");
    let again = trade_page_again(&app, &game).await;
    assert_eq!(
        (most(&again, "argent_donne"), most(&again, "argent_pris")),
        after
    );

    // Cancelling changes nothing.
    let coup = move_number(&again).expect("a decision");
    app.post(TRADE, &format!("coup={coup}&action=annuler"))
        .await;
    let again = trade_page_again(&app, &current_page(&app).await).await;
    assert_eq!(
        (most(&again, "argent_donne"), most(&again, "argent_pris")),
        after
    );
}

/// Opens the trade page again from the movement, and returns it.
async fn trade_page_again(app: &TestApp, game: &str) -> String {
    let coup = move_number(game).expect("a decision");
    let index = button(game, "Échanger avec").expect("trade button");
    app.post(
        &format!("{GAME}/reponse"),
        &format!("choix={index}&coup={coup}"),
    )
    .await;
    app.get(TRADE).await.body
}
