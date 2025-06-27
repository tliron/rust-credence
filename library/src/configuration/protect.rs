use super::super::resolve::*;

use {::axum::http::HeaderMap, compris::resolve::*, kutil_cli::debug::*, kutil_http::*, kutil_std::zerocopy::*};

//
// Protect
//

/// Protect.
#[derive(Clone, Debug, Debuggable, Default, Resolve)]
pub struct Protect {
    /// Regex.
    ///
    /// See [implementation syntax](https://docs.rs/regex/latest/regex/#syntax).
    #[resolve(required)]
    #[debuggable(option, as(display), style(string))]
    pub regex: Option<ResolveRegex>,

    /// Optional realm.
    ///
    /// Note that modern browsers do not display the realm.
    #[resolve]
    #[debuggable(option, style(string))]
    pub realm: Option<ByteString>,

    /// Username.
    #[resolve(required)]
    #[debuggable(style(string))]
    pub username: ByteString,

    /// Password.
    #[resolve(required)]
    #[debuggable(style(string))]
    pub password: ByteString,
}

impl Protect {
    /// True if protected.
    pub fn protect(&self, uri_path: &str) -> bool {
        if let Some(regex) = &self.regex
            && regex.inner.is_match(uri_path)
        {
            return true;
        }

        false
    }

    /// If the request is authorized returns [None].
    ///
    /// Otherwise returns a `WWW-Authenticate` header value.
    pub fn authorized(&self, headers: &HeaderMap) -> Option<ByteString> {
        if let Some((username, password)) = headers.authorization_basic()
            && (self.username == username)
            && (self.password == password)
        {
            match &self.realm {
                Some(realm) => tracing::debug!("authorized: {}", realm),
                None => tracing::debug!("authorized"),
            }
            return None;
        }

        let authenticate = match &self.realm {
            Some(realm) => {
                // Note that modern browsers do not display the realm
                tracing::debug!("unauthorized: {}", realm);
                &format!("Basic realm=\"{}\", charset=\"UTF-8\"", realm)
            }

            None => {
                tracing::debug!("unauthorized");
                "Basic charset=\"UTF-8\""
            }
        };

        Some(authenticate.into())
    }
}
