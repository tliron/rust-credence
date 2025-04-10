use {
    axum::{
        http::{HeaderMap, StatusCode, header::*},
        response::*,
    },
    compris::resolve::*,
    kutil_http::*,
    regex::*,
};

//
// Protect
//

/// Protect.
#[derive(Clone, Debug, Default, Resolve)]
pub struct Protection {
    /// Regex.
    #[resolve(required)]
    pub regex: Option<ResolveFromStr<Regex>>,

    /// Optional realm.
    ///
    /// Note that modern browsers do not display the realm.
    #[resolve]
    pub realm: Option<String>,

    /// Username.
    #[resolve(required)]
    pub username: String,

    /// Password.
    #[resolve(required)]
    pub password: String,
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

    /// Authorize.
    pub fn authorize(&self, headers: &HeaderMap) -> Result<(), Response> {
        if let Some((username, password)) = headers.authorization_basic() {
            if (username == self.username) && (password == self.password) {
                tracing::debug!("authorized");
                return Ok(());
            }
        }

        let authenticate = match &self.realm {
            Some(realm) => {
                tracing::debug!("unauthorized: {}", realm);
                &format!("Basic realm=\"{}\", charset=\"UTF-8\"", realm)
            }
            None => {
                tracing::debug!("unauthorized");
                "Basic charset=\"UTF-8\""
            }
        };

        Err((StatusCode::UNAUTHORIZED, [(WWW_AUTHENTICATE, authenticate)]).into_response())
    }
}
