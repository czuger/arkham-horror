/// `robots.txt` forbidding all crawling.
pub async fn robots() -> &'static str {
    "User-agent: *\nDisallow: /\n"
}
