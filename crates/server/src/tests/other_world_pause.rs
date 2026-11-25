//! A pause after the human's Other World encounter, even when it asked
//! nothing.

use std::path::Path;

use data::{GameData, InvestigatorId};
use engine::{Choice, DecisionKind, Place, Setup};

use crate::database::Database;
use crate::play_mode::PlayMode;
use crate::session::Session;
use crate::tests::test_app::TestApp;

/// The option that walks toward an open gate: into its location, else to
/// its street, else the farthest street offered; the first option for any
/// other decision.
fn toward_a_gate(data: &GameData, session: &Session) -> usize {
    let Some(decision) = session.decision() else {
        return 0;
    };
    if decision.kind != DecisionKind::Move {
        return 0;
    }
    let gates: Vec<_> = session
        .outcome
        .state
        .gates
        .iter()
        .map(|gate| gate.location.clone())
        .collect();
    let streets: Vec<_> = gates
        .iter()
        .filter_map(|id| data.board.location(id))
        .map(|location| location.street.clone())
        .collect();
    let options = &decision.options;
    options
        .iter()
        .position(|o| matches!(&o.choice, Choice::Place(Place::Location(id)) if gates.contains(id)))
        .or_else(|| {
            options.iter().position(
                |o| matches!(&o.choice, Choice::Place(Place::Street(id)) if streets.contains(id)),
            )
        })
        .or_else(|| {
            options
                .iter()
                .rposition(|o| matches!(&o.choice, Choice::Place(Place::Street(_))))
        })
        .unwrap_or(0)
}

/// Plays seed 3 until the human's next page is the Other World pause.
pub(crate) async fn play_into_an_other_world(db: &Database, data: &GameData) {
    let setup = Setup {
        seed: 3,
        players: 3,
        investigators: vec![InvestigatorId::new("amanda_sharpe")],
        ancient_one: None,
    };
    let mut session = Session::start(db, data, setup, PlayMode::Solo)
        .await
        .expect("start");
    for _ in 0..100 {
        let pick = toward_a_gate(data, &session);
        let move_number = session.move_number();
        session.answer(data, pick, move_number).expect("answer");
        if session.page().ends_with("/autre-monde") {
            session.save(db, data).await.expect("save");
            return;
        }
    }
    panic!("no Other World encounter");
}

#[tokio::test(flavor = "current_thread")]
async fn an_other_world_encounter_is_followed_by_a_pause() {
    let dir = tempfile::tempdir().expect("temp dir");
    let path = dir.path().join("db");
    let data = GameData::load(Path::new("../../data")).expect("data");
    let db = Database::open(&path).await.expect("db");
    play_into_an_other_world(&db, &data).await;
    db.close().await;

    let app = TestApp::open(&path).await;
    let page = app.get("/partie/1/autre-monde").await.body;
    assert!(page.contains("Amanda Sharpe — rencontre dans"), "{page}");
    assert!(page.contains("Carte Portail tirée par"), "{page}");
    assert!(page.contains("Continuer la partie"), "{page}");
}
