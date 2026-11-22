//! Relative links, so the application works under any URL prefix.
//!
//! Deployed, a proxy serves the application under a sub-path and strips it
//! before the request arrives: the server never knows that prefix. Every
//! link is therefore written from the application root without a leading
//! `/` (`partie/5/journal`), and each page declares, in `<base href>`, the
//! way back to that root from its own address.

use axum::http::Uri;
use axum::response::Redirect;

/// The way back from the request path `path` to the application root:
/// `./` for `/` or `/nouvelle`, `../` for `/partie/5`, `../../` for
/// `/partie/5/journal`…
pub fn root_of(path: &str) -> String {
    match path.trim_start_matches('/').matches('/').count() {
        0 => "./".to_owned(),
        depth => "../".repeat(depth),
    }
}

/// A redirect to `target`, a path from the application root without a
/// leading `/` (empty for the home page), written relative to the request
/// `from`.
pub fn redirect_from(from: &Uri, target: &str) -> Redirect {
    Redirect::to(&format!("{}{target}", root_of(from.path())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roots_climb_one_level_per_folder() {
        assert_eq!(root_of("/"), "./");
        assert_eq!(root_of("/nouvelle"), "./");
        assert_eq!(root_of("/partie/5"), "../");
        assert_eq!(root_of("/partie/5/journal"), "../../");
        assert_eq!(root_of("/partie/5/investigateurs/2"), "../../../");
    }
}
