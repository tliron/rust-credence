use super::super::resolve::*;

use {
    axum::http::*,
    compris::resolve::*,
    kutil::{cli::depict::*, std::immutable::*},
    regex::*,
};

//
// Redirect
//

/// Redirect.
#[derive(Clone, Debug, Depict, Resolve)]
pub struct Redirect {
    /// Regex.
    ///
    /// See [implementation syntax](https://docs.rs/regex/latest/regex/#syntax).
    #[resolve(required)]
    #[depict(as(display), style(string))]
    pub regex: ResolveRegex,

    /// Expand to.
    ///
    /// See [implementation syntax](https://docs.rs/regex/latest/regex/struct.Captures.html#method.expand).
    #[resolve(required, key = "to")]
    #[depict(style(string))]
    pub expand_to: ByteString,

    /// Redirect status code. Defaults to 301 (Moved Permanently).
    #[resolve(key = "code")]
    #[depict(as(display), style(symbol))]
    pub status_code: ResolveStatusCode,
}

impl Redirect {
    /// If the URI is redirected returns the redirected URI.
    pub fn redirect(&self, uri_path: &str) -> Option<(String, StatusCode)> {
        if let Some(captures) = self.regex.inner.captures(uri_path) {
            let mut uri_path = String::default();
            captures.expand(&self.expand_to, &mut uri_path);
            return Some((uri_path, self.status_code.inner));
        }

        None
    }
}

impl Default for Redirect {
    fn default() -> Self {
        Self {
            regex: Regex::new("").expect("regex").into(),
            expand_to: Default::default(),
            status_code: StatusCode::MOVED_PERMANENTLY.into(),
        }
    }
}
