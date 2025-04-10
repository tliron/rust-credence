use super::super::configuration::*;

use {
    axum::{extract::*, http::StatusCode, middleware::*, response::*},
    kutil_http::file::*,
    std::path::*,
};

//
// StatusMiddleware
//

/// Axum middleware that renders non-success status codes to files.
#[derive(Clone, Debug)]
pub struct StatusMiddleware {
    /// Assets path.
    pub assets_path: PathBuf,
}

impl StatusMiddleware {
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
        let response = next.run(request).await;

        let status = response.status();
        if !status.is_success() {
            let status = status.as_u16();
            let file_path = state_self
                .assets_path
                .join(status.to_string() + HTML_SUFFIX);
            if file_path.exists() {
                tracing::debug!("status page: {}", status);
                return Ok(response_from_file(file_path).await.into_response());
            }
        }

        Ok(response)
    }
}
