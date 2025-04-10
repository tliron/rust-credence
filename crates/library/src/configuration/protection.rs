use super::super::parse::*;

use {axum::http::HeaderMap, bytestring::*, compris::resolve::*, kutil_http::*};

//
// Protect
//

/// Protect.
#[derive(Clone, Debug, Default, Resolve)]
pub struct Protection {
    /// Regex.
    ///
    /// See [implementation syntax](https://docs.rs/regex/latest/regex/#syntax).
    #[resolve(required)]
    pub regex: Option<ResolveRegex>,

    /// Optional realm.
    ///
    /// Note that modern browsers do not display the realm.
    #[resolve]
    pub realm: Option<ByteString>,

    /// Username.
    #[resolve(required)]
    pub username: ByteString,

    /// Password.
    #[resolve(required)]
    pub password: ByteString,
}

impl Protection {
    /// True if protected.
    pub fn protected(&self, uri_path: &str) -> bool {
        if let Some(regex) = &self.regex {
            if regex.value.is_match(uri_path) {
                return true;
            }
        }

        false
    }

    /// If the request is authorized returns [None](Option::None).
    ///
    /// Otherwise returns a `WWW-Authenticate` header value.
    pub fn authorized(&self, headers: &HeaderMap) -> Option<ByteString> {
        if let Some((username, password)) = headers.authorization_basic() {
            if (self.username == username) && (self.password == password) {
                match &self.realm {
                    Some(realm) => tracing::debug!("authorized: {}", realm),
                    None => tracing::debug!("authorized"),
                }
                return None;
            }
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
