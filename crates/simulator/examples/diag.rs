//! Temporary diagnostics for AI tuning.
#![allow(missing_docs, clippy::all)]

use std::collections::BTreeMap;
use std::path::PathBuf;

use ai::Policy;
use data::GameData;
use engine::{Decision, Phase, Place, Responder, Setup, View, play};

struct Spy {
    policy: Policy,
    last_turn: u32,
    reasons: BTreeMap<String, u64>,
    awake: Option<(u8, u32, String)>,
    min_gates: usize,
}

impl Responder for Spy {
    fn answer(&mut self, view: &View<'_, '_>, decision: &Decision) -> Option<usize> {
        let state = view.state();
        if state.turn != self.last_turn && state.battle.is_none() {
            self.last_turn = state.turn;
            for seat in 0..state.investigators.len() {
                let inv = &state.investigators[seat];
                let key = match &inv.place {
                    _ if inv.eliminated => "éliminé".to_owned(),
                    Place::OtherWorld { .. } => "autre monde".to_owned(),
                    Place::LostInTimeAndSpace => "perdu".to_owned(),
                    Place::JailCell => "prison".to_owned(),
                    _ if inv.delayed => "retardé".to_owned(),
                    _ => match ai::intention(view, seat) {
                        Some((target, r)) => {
                            let d = view.distance(&inv.place, &target).unwrap_or(99);
                            let mv = view.movement_points(seat);
                            if format!("{r:?}") == "Gate" {
                                format!("Gate d={d} mv={}", mv.min(9))
                            } else {
                                format!("{r:?}")
                            }
                        }
                        None => "None".to_owned(),
                    },
                };
                *self.reasons.entry(key).or_insert(0) += 1;
            }
        }
        if state.turn >= 3 && state.battle.is_none() {
            self.min_gates = self.min_gates.min(state.gates.len());
        }
        if state.battle.is_some() && self.awake.is_none() {
            let dice: u32 = (0..state.investigators.len())
                .filter(|&s| !state.investigators[s].eliminated)
                .map(|s| view.check_dice(s, data::CheckKind::Combat, 0))
                .sum();
            let alive = state.investigators.iter().filter(|i| !i.eliminated).count();
            self.awake = Some((state.doom, dice, format!("{alive}")));
        }
        let _ = Phase::Upkeep;
        self.policy.answer(view, decision)
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let games: u64 = args.get(1).and_then(|a| a.parse().ok()).unwrap_or(300);
    let players: usize = args.get(2).and_then(|a| a.parse().ok()).unwrap_or(3);
    let data = GameData::load(&PathBuf::from("data")).unwrap();
    let mut reasons = BTreeMap::new();
    let mut rounds = 0u64;
    let mut battles = 0u64;
    let mut doom_left = 0u64;
    let mut doom_start = 0u64;
    let mut dice = 0u64;
    let mut wins = 0u64;
    let mut clues_end = 0u64;
    let mut endings = BTreeMap::new();
    let mut mins = BTreeMap::new();
    let (mut closed, mut sealed, mut kos) = (0u64, 0u64, 0u64);
    let detail = std::env::var("DETAIL").is_ok();
    for seed in 1..=games {
        let mut spy = Spy {
            policy: Policy::new(),
            last_turn: u32::MAX,
            reasons: BTreeMap::new(),
            awake: None,
            min_gates: 99,
        };
        let outcome = play(&data, &Setup::random(seed, players), &mut spy).unwrap();
        for (k, v) in spy.reasons {
            *reasons.entry(k).or_insert(0) += v;
        }
        if outcome.state.ending.is_some_and(|e| e.is_victory()) {
            wins += 1;
        }
        *endings
            .entry(format!("{:?}", outcome.state.ending))
            .or_insert(0u64) += 1;
        *mins.entry(spy.min_gates.min(4)).or_insert(0u64) += 1;
        closed += u64::from(outcome.tally.gates_closed);
        sealed += u64::from(outcome.tally.gates_sealed);
        kos += u64::from(outcome.tally.knocked_out);
        if let Some((doom, d, _)) = spy.awake {
            battles += 1;
            doom_start += u64::from(doom);
            dice += u64::from(d);
            doom_left += u64::from(outcome.state.doom);
            rounds += u64::from(outcome.state.battle.as_ref().map_or(0, |b| b.round));
        }
        clues_end += outcome
            .state
            .investigators
            .iter()
            .map(|i| u64::from(i.clues))
            .sum::<u64>();
    }
    let total: u64 = reasons.values().sum();
    let g = games as f64;
    println!(
        "fermés {:.2} scellés {:.2} KO {:.2} | fins {:?} | min portails (t>=3) {:?}",
        closed as f64 / g,
        sealed as f64 / g,
        kos as f64 / g,
        endings,
        mins
    );
    for (k, v) in reasons.iter().filter(|_| detail) {
        println!("{:>6.1} %  {k}", 100.0 * *v as f64 / total as f64);
    }
    let b = battles.max(1) as f64;
    println!(
        "victoires {wins}; batailles {battles}: destin départ {:.1}, restant {:.1}, rounds {:.1}, dés de Vigueur (sans armes) au réveil {:.1}; indices en fin {:.1}",
        doom_start as f64 / b,
        doom_left as f64 / b,
        rounds as f64 / b,
        dice as f64 / b,
        clues_end as f64 / games as f64
    );
}
