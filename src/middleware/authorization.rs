#![allow(dead_code)]

use {
    axum::{
        extract::*,
        http::{StatusCode, header::*},
        middleware::*,
        response::*,
    },
    kutil_http::*,
};

//
// BasicAuthorizationMiddleware
//

/// Basic authorization middleware.
#[derive(Clone, Debug)]
pub struct BasicAuthorizationMiddleware {
    /// Realm.
    pub realm: String,

    /// Username.
    pub username: String,

    /// Password.
    pub password: String,
}

impl BasicAuthorizationMiddleware {
    /// Constructor.
    pub fn new(realm: String, username: String, password: String) -> Self {
        Self {
            realm,
            username,
            password,
        }
    }

    /// To be used with [from_fn_with_state].
    pub async fn function(
        State(state_self): State<Self>,
        request: Request,
        next: Next,
    ) -> Result<Response, Response> {
        if let Some((username, password)) = request.headers().authorization_basic() {
            if (username == state_self.username) && (password == state_self.password) {
                tracing::debug!("authorized");
                return Ok(next.run(request).await);
            }
        }

        tracing::debug!("unauthorized: {}", state_self.realm);

        // Note that modern browsers do not display the realm
        let authenticate = format!("Basic realm=\"{}\", charset=\"UTF-8\"", state_self.realm);
        Err((StatusCode::UNAUTHORIZED, [(WWW_AUTHENTICATE, authenticate)]).into_response())
    }
}
