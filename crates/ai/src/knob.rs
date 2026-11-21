//! TEMPORARY tuning knobs read from the environment.
#![allow(missing_docs)]

use std::collections::BTreeMap;
use std::sync::OnceLock;

static KNOBS: OnceLock<BTreeMap<String, f64>> = OnceLock::new();

pub fn knob(name: &str, default: f64) -> f64 {
    KNOBS
        .get_or_init(|| {
            std::env::vars()
                .filter_map(|(k, v)| Some((k.strip_prefix("K_")?.to_owned(), v.parse().ok()?)))
                .collect()
        })
        .get(name)
        .copied()
        .unwrap_or(default)
}
