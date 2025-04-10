use super::{
    super::{configuration::*, render::*},
    constants::*,
};

use {
    axum::{
        extract::*,
        http::{StatusCode, header::*},
        response::*,
    },
    kutil_http::*,
};

//
// UriMiddleware
//

/// Axum middleware that rewrites URI paths.
///
/// * Redirects trailing slashes (301)
/// * Hides paths (404) that end with ".html", ".html.md", or other configured suffixes
/// * Rewrites to ".html" or ".html.md" files if they exist
/// * Otherwise if it's a directory rewrites to "index.html" or "index.html.md" if they exist in it
#[derive(Clone, Debug)]
pub struct UriMiddleware {
    /// Configuration.
    pub configuration: ServerConfiguration,
}

impl UriMiddleware {
    /// Constructor.
    pub fn new(configuration: ServerConfiguration) -> Self {
        Self { configuration }
    }

    /// To be used with [map_request_with_state].
    pub async fn function(
        State(state_self): State<Self>,
        mut request: Request,
    ) -> Result<Request, Response> {
        let original_path = request.uri().path();
        let mut path = original_path;
        let mut new_path = None;

        // Redirect

        if let Some(path) = state_self.configuration.uri.redirect(path) {
            // Note that since RFC 7231 relative `Location` is allowed
            tracing::info!("redirect to: {}", path);
            return Err((StatusCode::MOVED_PERMANENTLY, [(LOCATION, path)]).into_response());
        }

        // Hide

        if state_self.configuration.is_hidden(path) {
            return Err(StatusCode::NOT_FOUND.into_response());
        }

        // Protect

        if let Some(protection) = state_self.configuration.uri.protection(path) {
            protection.authorize(request.headers())?;
        }

        // Rewrite

        let mut asset_path = state_self.configuration.paths.asset(path);

        // If it's a directory then switch to the index file in the directory
        let mut _path = String::default();
        if asset_path.is_dir() {
            asset_path = asset_path.join(INDEX);
            _path = path.trim_end_matches(PATH_SEPARATOR).to_string() + SEPARATOR_AND_INDEX;
            path = &_path;
        }

        let html_file = asset_path.with_extension(HTML_EXTENSION);
        if html_file.exists() {
            // {path}.html
            new_path = Some(String::from(path) + HTML_SUFFIX);
        } else if let Some(render_uri_path) = state_self
            .configuration
            .rendered_page_uri_path(path)
            .map_err_internal_server("rendered page URI path")
            .map_err(|error| error.into_response())?
        {
            // {path}.r.*
            new_path = Some(render_uri_path);
        }

        if let Some(new_path) = new_path {
            tracing::debug!("rewriting {} to {}", original_path, new_path);
            request
                .set_uri_path(&new_path)
                .map_err_internal_server("set URI path")
                .map_err(|error| error.into_response())?;
        }

        Ok(request)
    }
}
