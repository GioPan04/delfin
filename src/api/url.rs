pub fn httpify(url: &str) -> String {
    let url = url.strip_suffix('/').unwrap_or(url);
    if !(url.starts_with("https://") || url.starts_with("http://")) {
        return format!("https://{}", url);
    }
    url.to_string()
}
