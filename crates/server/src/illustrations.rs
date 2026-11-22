//! The scene illustrations, shrunk at build time (`build.rs`) and built into
//! the program.
//!
//! Pages link them with a fingerprint (`?v=<fingerprint>`): a link with the
//! current fingerprint may be cached by the browser for a year.

use axum::body::Body;
use axum::extract::{Path, RawQuery};
use axum::http::{HeaderMap, HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Response};

use crate::embedded_files::serve_bytes;

/// The shrunk illustrations, `(path, bytes, fingerprint)`, sorted by path:
/// `grand/…` and `vignette/…`, JPEG. Written by `build.rs`.
static FILES: &[(&str, &[u8], &str)] = include!(concat!(env!("OUT_DIR"), "/illustration_files.rs"));

/// Which size of an illustration a page shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IllustrationSize {
    /// The scene: 1280 px on its longest side.
    Large,
    /// A thumbnail: 320 px on its longest side.
    Thumbnail,
}

impl IllustrationSize {
    fn folder(self) -> &'static str {
        match self {
            Self::Large => "grand",
            Self::Thumbnail => "vignette",
        }
    }
}

/// Finds a shrunk file by its path: its bytes and fingerprint.
fn find(path: &str) -> Option<(&'static [u8], &'static str)> {
    let index = FILES
        .binary_search_by(|(key, _, _)| (*key).cmp(path))
        .ok()?;
    FILES
        .get(index)
        .map(|(_, bytes, fingerprint)| (*bytes, *fingerprint))
}

/// The link to one illustration, relative to the site root.
///
/// # Arguments
///
/// * `size` - large or thumbnail.
/// * `source` - the path cited by the data, relative to `illustrations/`
///   (e.g. `"autres mondes/rlyeh_2.png"`).
///
/// # Returns
///
/// `illustrations/<size>/<path>.jpg?v=<fingerprint>`, spaces encoded.
pub fn illustration_url(size: IllustrationSize, source: &str) -> String {
    let stem = source.strip_suffix(".png").unwrap_or(source);
    let path = format!("{}/{stem}.jpg", size.folder());
    let encoded = path.replace(' ', "%20");
    match find(&path) {
        Some((_, fingerprint)) => format!("illustrations/{encoded}?v={fingerprint}"),
        None => format!("illustrations/{encoded}"),
    }
}

/// Serves one illustration; with the current fingerprint, cacheable for a year.
pub async fn illustration(
    Path(path): Path<String>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
) -> Response {
    let Some((bytes, fingerprint)) = find(&path) else {
        return StatusCode::NOT_FOUND.into_response();
    };
    let mut response = serve_bytes(Body::from(bytes), "image/jpeg", fingerprint, &headers);
    if query.as_deref() == Some(format!("v={fingerprint}").as_str()) {
        response.headers_mut().insert(
            header::CACHE_CONTROL,
            HeaderValue::from_static("public, max-age=31536000, immutable"),
        );
    }
    response
}
