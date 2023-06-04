pub fn httpify(url: &str) -> String {
    let url = url.strip_suffix('/').unwrap_or(url).to_string();

    // Add protocol if not already there
    let url = if !(url.starts_with("https://") || url.starts_with("http://")) {
        format!("https://{}", url)
    } else {
        url
    };

    // Ensure there's a slash at the end of the path, otherwise reqwest will
    // drop the end when joining URLs
    if !(url.ends_with('/')) {
        format!("{}/", url)
    } else {
        url
    }
}
