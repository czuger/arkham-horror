//! Coverage of the cards over many games.

use std::collections::BTreeSet;
use std::path::Path;

use data::GameData;

/// Prints the share of cards and paragraphs played, by kind, and writes the
/// ones never played to `missing_file` when given.
pub fn report(
    data: &GameData,
    covered: &BTreeSet<String>,
    missing_file: Option<&Path>,
) -> Result<(), String> {
    let expected = engine::coverage_keys(data);
    let missing: Vec<&String> = expected.difference(covered).collect();
    println!("Couverture :");
    for kind in ["Lieu", "Portail", "Mythe", "Carte reçue", "Carte utilisée"] {
        let total = expected.iter().filter(|key| is_kind(key, kind)).count();
        let left = missing.iter().filter(|key| is_kind(key, kind)).count();
        println!("  {kind:<15} {:>4} / {total}", total - left);
    }
    let Some(path) = missing_file else {
        return Ok(());
    };
    let text: String = missing.iter().map(|key| format!("{key}\n")).collect();
    std::fs::write(path, text)
        .map_err(|error| format!("écriture de {} impossible : {error}", path.display()))?;
    println!(
        "Jamais joués : {} (liste dans {}).",
        missing.len(),
        path.display()
    );
    Ok(())
}

/// Whether a coverage key is of a kind (its first words).
fn is_kind(key: &str, kind: &str) -> bool {
    key.strip_prefix(kind)
        .is_some_and(|rest| rest.starts_with(' '))
}
