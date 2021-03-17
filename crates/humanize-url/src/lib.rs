pub use unrestrictive_url::ParseError;
use unrestrictive_url::{UnrestrictiveUrl, Url};

pub fn humanize_url(url: &str) -> Result<String, ParseError> {
    let url = Url::parse(url)?;
    let mut url = UnrestrictiveUrl::from(&url);

    // Remove protocol.
    url.scheme = None;
    // Remove authentication.
    url.username = None;
    url.password = None;

    // Remove trailing slashes.
    let url = url.to_string();
    let mut chars = url.chars();
    if chars.next_back() == Some('/') {
        Ok(chars.collect())
    } else {
        Ok(url)
    }
}

#[cfg(test)]
mod tests {
    use super::humanize_url;

    #[test]
    fn removes_scheme() {
        let url = humanize_url("https://github.com/SirWindfield").unwrap();
        assert_eq!("github.com/SirWindfield", url);
    }

    #[test]
    fn removes_trailing_slash() {
        let url = humanize_url("https://github.com/SirWindfield/").unwrap();
        assert_eq!("github.com/SirWindfield", url);
    }

    #[test]
    fn removes_authentication() {
        let url = humanize_url("https://user:pw@github.com/SirWindfield").unwrap();
        assert_eq!("github.com/SirWindfield", url);
    }
}
