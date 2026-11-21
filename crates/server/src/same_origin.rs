use axum::extract::Request;
use axum::http::{HeaderMap, Method, header};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

use crate::app_error::AppError;

/// Refuses forms posted from another site (cross-site request forgery).
///
/// Browsers say where a request comes from in `Sec-Fetch-Site` and
/// `Origin`; a POST is accepted only when they match this site or are
/// absent (old browsers, command-line tools).
pub async fn same_origin(request: Request, next: Next) -> Response {
    if request.method() == Method::POST && !is_same_origin(request.headers()) {
        return AppError::Forbidden.into_response();
    }
    next.run(request).await
}

/// Whether the request headers show a request from this site.
fn is_same_origin(headers: &HeaderMap) -> bool {
    let text = |name| headers.get(name).and_then(|value| value.to_str().ok());
    if let Some(site) = text(header::HeaderName::from_static("sec-fetch-site")) {
        return matches!(site, "same-origin" | "none");
    }
    match (text(header::ORIGIN), text(header::HOST)) {
        (Some(origin), Some(host)) => origin
            .split_once("://")
            .is_some_and(|(_, rest)| rest == host),
        _ => true,
    }
}

#[cfg(test)]
mod tests {
    use axum::http::HeaderValue;

    use super::*;

    fn headers(pairs: &[(&'static str, &'static str)]) -> HeaderMap {
        let mut map = HeaderMap::new();
        for (name, value) in pairs {
            map.insert(*name, HeaderValue::from_static(value));
        }
        map
    }

    #[test]
    fn cross_site_posts_are_refused() {
        assert!(!is_same_origin(&headers(&[(
            "sec-fetch-site",
            "cross-site"
        )])));
        assert!(!is_same_origin(&headers(&[
            ("origin", "http://evil.example"),
            ("host", "127.0.0.1:3000"),
        ])));
    }

    #[test]
    fn same_site_posts_are_accepted() {
        assert!(is_same_origin(&headers(&[(
            "sec-fetch-site",
            "same-origin"
        )])));
        assert!(is_same_origin(&headers(&[
            ("origin", "http://127.0.0.1:3000"),
            ("host", "127.0.0.1:3000"),
        ])));
        assert!(is_same_origin(&HeaderMap::new()));
    }
}
