use std::{fmt, str::Split};
pub use url::*;

/// A small wrapper around [`url::Url`] that allows free URL modifications.
///
/// Since the [`url`] crate strictly follows the [WHATWG](https://url.spec.whatwg.org/) specification, some operations are deemed illegal and can't be performed with the crate. This crate allows such operations.
///
/// # Example
///
/// ```rust
/// use unrestrictive_url::{Url, UnrestrictiveUrl};
///
/// let url = Url::parse("https://github.com").unwrap();
/// let mut url = UnrestrictiveUrl::from(&url);
/// url.scheme = Some("jojo");
///
/// assert_eq!("jojo://github.com/", url.to_string());
/// ```
pub struct UnrestrictiveUrl<'a> {
    pub fragment: Option<&'a str>,
    pub host: Option<url::Host<&'a str>>,
    pub password: Option<&'a str>,
    pub path: Option<&'a str>,
    pub port: Option<u16>,
    pub query: Option<&'a str>,
    pub scheme: Option<&'a str>,
    pub username: Option<&'a str>,
    // Probably not needed but I kept it in to make it fully compliant with the specification's
    // serialization description.
    cannot_be_a_base: bool,
}

impl<'a> UnrestrictiveUrl<'a> {
    pub fn path_segments(&self) -> Option<Split<'a, char>> {
        self.path.and_then(|v| {
            if v.starts_with('/') {
                Some(v[1..].split('/'))
            } else {
                None
            }
        })
    }
}

impl fmt::Display for UnrestrictiveUrl<'_> {
    // https://url.spec.whatwg.org/#url-serializing
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 1)
        if let Some(scheme) = self.scheme {
            // In reality URLs have to have a schema. But for special use-cases like URL
            // truncation, an empty schema might be desireable.
            write!(f, "{}:", scheme)?;
        }

        // 2)
        if self.host.is_some() {
            // 2.1)
            // XXX: special case for no scheme. In these cases, a double slash is probably
            // not wanted. The `url` crate won't parse URLs starting with a double slash
            // anyway without having a base URL specified, which this crate does not allow
            // to do (and probably won't ever).
            if self.scheme.is_some() {
                write!(f, "//")?;
            }

            if let Some(username) = self.username {
                // 2.2.1)
                write!(f, "{}", username)?;
                if let Some(password) = self.password {
                    if !password.is_empty() {
                        // 2.2.2)
                        write!(f, ":{}", password)?;
                    }
                }

                // 2.2.3)
                write!(f, "@")?;
            }

            // 2.3)
            match &self.host {
                Some(host) => match host {
                    url::Host::Domain(v) => write!(f, "{}", v)?,
                    url::Host::Ipv4(v) => write!(f, "{}", v)?,
                    url::Host::Ipv6(v) => write!(f, "[{}]", v)?,
                },
                None => {}
            }

            // 2.4)
            if let Some(port) = self.port {
                write!(f, ":{}", port)?;
            }
        }

        // 3)
        if self.cannot_be_a_base {
            let first_path_segment = self.path_segments().and_then(|mut v| v.next());
            if let Some(segment) = first_path_segment {
                write!(f, "{}", segment)?;
            }
        } else {
            // 4)
            if let Some(path) = self.path {
                // Special case '/' only.
                if path == "/" {
                    write!(f, "/")?;
                } else {
                    let path_segments = path.split('/').collect::<Vec<_>>();
                    if self.host.is_none() && path_segments.len() > 1 && path_segments[0] == "" {
                        write!(f, "/.")?;
                    }

                    for segment in path_segments {
                        write!(f, "/{}", segment)?;
                    }
                }
            }
        }

        // 5)
        if let Some(query) = self.query {
            write!(f, "?{}", query)?;
        }

        // 6)
        if let Some(fragment) = self.fragment {
            write!(f, "#{}", fragment)?;
        }

        Ok(())
    }
}

impl<'a> From<&'a url::Url> for UnrestrictiveUrl<'a> {
    fn from(url: &'a url::Url) -> Self {
        let username = if url.username().is_empty() {
            None
        } else {
            Some(url.username())
        };

        Self {
            fragment: url.fragment(),
            host: url.host(),
            password: url.password(),
            path: url.path().into(),
            port: url.port(),
            query: url.query(),
            scheme: url.scheme().into(),
            username,
            cannot_be_a_base: url.cannot_be_a_base(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{UnrestrictiveUrl, Url};

    #[test]
    fn test_arbitrary_scheme() {
        let url = "https://github.com";
        let url = Url::parse(url).unwrap();
        let mut url: UnrestrictiveUrl = (&url).into();
        url.scheme = Some("github");

        assert_eq!("github://github.com/", url.to_string());
    }

    #[test]
    fn test_remove_scheme() {
        let url = "https://github.com";
        let url = Url::parse(url).unwrap();
        let mut url: UnrestrictiveUrl = (&url).into();
        url.scheme = None;

        assert_eq!("github.com/", url.to_string());
    }

    #[test]
    fn test_remove_fragment() {
        let url = "https://github.com#fragment";
        let url = Url::parse(url).unwrap();
        let mut url: UnrestrictiveUrl = (&url).into();
        url.fragment = None;

        assert_eq!("https://github.com/", url.to_string());
    }

    #[test]
    fn test_remove_query() {
        let url = "https://github.com?q=search&otherstuff=5";
        let url = Url::parse(url).unwrap();
        let mut url: UnrestrictiveUrl = (&url).into();
        url.query = None;

        assert_eq!("https://github.com/", url.to_string());
    }

    #[test]
    fn test_remove_password() {
        let url = "https://user:pw@github.com";
        let url = Url::parse(url).unwrap();
        let mut url: UnrestrictiveUrl = (&url).into();
        url.password = None;

        assert_eq!("https://user@github.com/", url.to_string());
    }

    #[test]
    fn test_remove_username() {
        let url = "https://user:pw@github.com";
        let url = Url::parse(url).unwrap();
        let mut url: UnrestrictiveUrl = (&url).into();
        url.username = None;

        assert_eq!("https://github.com/", url.to_string());
    }
}
