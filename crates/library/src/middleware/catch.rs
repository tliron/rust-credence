use super::{constants::*, defer::*};

use {
    axum::{
        extract::{Request, *},
        http::*,
        middleware::*,
        response::{Response, *},
    },
    kutil_http::file::*,
    std::{path::*, result::Result},
};

//
// CatchMiddleware
//

/// Axum middleware that "catches" ...
///
/// * [DeferredResponse], which it finally handles, and
/// * any non-success [StatusCode], for which it generates a response from a file if its exists
#[derive(Clone, Debug)]
pub struct CatchMiddleware {
    /// Assets path.
    pub assets_path: PathBuf,
}

impl CatchMiddleware {
    /// Constructor.
    pub fn new(assets_path: PathBuf) -> Self {
        Self { assets_path }
    }

    /// To be used with [from_fn_with_state].
    pub async fn function(
        State(state_self): State<Self>,
        request: Request,
        next: Next,
    ) -> Result<Response, StatusCode> {
        if let Some(deferred_response) = DeferredResponse::get(&request) {
            match deferred_response {
                DeferredResponse::Hide => {
                    if let Some(response) = state_self.response(StatusCode::NOT_FOUND).await {
                        return Ok(response);
                    }
                }

                DeferredResponse::Authenticate(authenticate) => {
                    return Ok(DeferredResponse::authenticate(authenticate));
                }

                DeferredResponse::RedirectTo((uri_path, status_code)) => {
                    return Ok(DeferredResponse::redirect_to(uri_path, *status_code));
                }

                DeferredResponse::Error(message) => {
                    tracing::error!("{}", message);
                    if let Some(response) = state_self.response(StatusCode::INTERNAL_SERVER_ERROR).await {
                        return Ok(response);
                    }
                }

                _ => {}
            }
        }

        let response = next.run(request).await;

        let status = response.status();
        if !status.is_success() {
            if let Some(response) = state_self.response(status).await {
                return Ok(response);
            }
        }

        Ok(response)
    }

    /// Generate a response for a [StatusCode].
    pub async fn response(&self, status: StatusCode) -> Option<Response> {
        let status = status.as_u16();
        let file_path = self.assets_path.join(status.to_string() + HTML_SUFFIX);
        if file_path.exists() {
            tracing::debug!("status page: {}", status);
            return Some(response_from_file(file_path, false).await.into_response());
        }

        None
    }
}
