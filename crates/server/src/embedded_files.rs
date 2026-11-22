//! The game data, images, CSS and JavaScript built into the program.
//!
//! A release build carries every file, so the deployed binary needs no
//! folder next to it. A debug build reads them from the repository at each
//! request instead (rust-embed's behaviour), so edits show without a rebuild.

use std::io;
use std::path::PathBuf;

use axum::body::Body;
use axum::extract::Path;
use axum::http::{HeaderMap, HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Response};
use data::{DataError, GameData};
use rust_embed::{EmbeddedFile, RustEmbed};

/// The game images (`assets/` in the repository).
#[derive(RustEmbed)]
#[folder = "../../assets"]
#[exclude = "*.md"]
pub struct Assets;

/// The CSS and JavaScript files (`static/` in the repository).
#[derive(RustEmbed)]
#[folder = "../../static"]
pub struct StaticFiles;

/// The game data files (`data/` in the repository).
#[derive(RustEmbed)]
#[folder = "../../data"]
#[include = "*.ron"]
pub struct GameFiles;

/// Loads and checks the built-in game data.
///
/// # Errors
///
/// Any [`DataError`]: missing or malformed file, unsupported version, or
/// inconsistent data.
pub fn load_game_data() -> Result<GameData, DataError> {
    GameData::load_with(|file_name| {
        let file = GameFiles::get(file_name).ok_or_else(|| missing(file_name))?;
        String::from_utf8(file.data.into_owned()).map_err(|error| DataError::Io {
            path: PathBuf::from(file_name),
            source: io::Error::new(io::ErrorKind::InvalidData, error),
        })
    })
}

fn missing(file_name: &str) -> DataError {
    DataError::Io {
        path: PathBuf::from(file_name),
        source: io::ErrorKind::NotFound.into(),
    }
}

/// Serves one built-in image.
pub async fn asset(Path(path): Path<String>, headers: HeaderMap) -> Response {
    serve(Assets::get(&path), &headers)
}

/// Serves one built-in CSS or JavaScript file.
pub async fn static_file(Path(path): Path<String>, headers: HeaderMap) -> Response {
    serve(StaticFiles::get(&path), &headers)
}

/// The file with its type and an `ETag`, `304` when the browser already has
/// it, `404` when there is no such file.
fn serve(file: Option<EmbeddedFile>, request: &HeaderMap) -> Response {
    let Some(file) = file else {
        return StatusCode::NOT_FOUND.into_response();
    };
    let etag = format!("\"{}\"", hex(&file.metadata.sha256_hash()));
    let mimetype = HeaderValue::from_str(file.metadata.mimetype());
    let fresh = request
        .get(header::IF_NONE_MATCH)
        .is_some_and(|value| value.as_bytes() == etag.as_bytes());
    let mut response = if fresh {
        StatusCode::NOT_MODIFIED.into_response()
    } else {
        Body::from(file.data).into_response()
    };
    let headers = response.headers_mut();
    if let Ok(value) = mimetype {
        headers.insert(header::CONTENT_TYPE, value);
    }
    if let Ok(value) = HeaderValue::from_str(&etag) {
        headers.insert(header::ETAG, value);
    }
    response
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
