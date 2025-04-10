use super::{
    super::{configuration::*, render::*, util::*},
    constants::*,
    defer::*,
};

use {::axum::extract::*, kutil_http::*};

//
// FacadeMiddleware
//

/// Axum middleware that handles the URI facade.
///
/// * Redirects trailing slashes (301)
/// * Hides paths (404) that end with `.html`, `.r.*`, or other configured suffixes
/// * Rewrites to `.html` or `.r.*` files if they exist
/// * Otherwise if it's a directory rewrites to `index.html` or `index.r.*` if they exist in it
///
/// All of the above is handled by attaching a [DeferredResponse] to the *request* (not the
/// response). We do this because request mapping middleware cannot return a normal response. Thus,
/// we also need to install [CatchMiddleware], which "catches" the deferment and generates the
/// actual response.
///
/// (Remember that axum runs through layers in the *reverse* order in which they are
/// programatically added, so add [CatchMiddleware] first *and then* add [FacadeMiddleware].)
///
/// We referred to "normal" responses above, because actually request mapping can return "abnormal"
/// responses: as a [Result::Err]. We can consider them abnormal because they circumvent the
/// remaining middleware, thus making that codepath unsuitable for us.
#[derive(Clone, Debug)]
pub struct FacadeMiddleware {
    /// Configuration.
    pub configuration: CredenceConfiguration,
}

impl FacadeMiddleware {
    /// Constructor.
    pub fn new(configuration: CredenceConfiguration) -> Self {
        Self { configuration }
    }

    /// To be used with [map_request_with_state].
    pub async fn function(State(state_self): State<Self>, mut request: Request) -> Request {
        let uri_path = match request.uri().decoded_path() {
            Some(uri_path) => uri_path,
            None => {
                // Cannot decode path
                return request;
            }
        };

        let original_uri_path = uri_path.clone();
        let mut uri_path = uri_path.as_ref();
        let mut new_uri_path = None;

        // Redirect

        if let Some((uri_path, status_code)) = state_self.configuration.urls.redirect(uri_path) {
            // Note that since RFC 7231 relative `Location` is allowed
            tracing::info!("redirect to: {} {}", status_code.as_u16(), uri_path);
            return request.with_deferred_redirect_to(uri_path.into(), status_code);
        }

        // Hide

        if state_self.configuration.hide(uri_path) {
            tracing::info!("hide: {}", uri_path);
            return request.with_deferred_hide();
        }

        // Protect

        if let Some(protect) = state_self.configuration.urls.protect(uri_path) {
            if let Some(authenticate) = protect.authorized(request.headers()) {
                return request.with_deferred_authenticate(authenticate);
            }
        }

        // Rewrite

        let mut asset_path = state_self.configuration.files.asset(uri_path);

        // If it's a directory then switch to the index file in the directory
        let mut _uri_path = String::default();
        if asset_path.is_dir() {
            asset_path = asset_path.join(INDEX);
            _uri_path = uri_path_join(uri_path, INDEX);
            uri_path = &_uri_path;
        }

        let html_file = asset_path.with_extension(HTML_EXTENSION);
        if html_file.exists() {
            // {path}.html
            new_uri_path = Some(String::from(uri_path) + HTML_SUFFIX);
        } else if let Some(rendered_page_uri_path) = match state_self.configuration.rendered_page_uri_path(uri_path) {
            Ok(uri_path) => uri_path,
            Err(_error) => {
                return request.with_deferred_error("rendered page URI path".into());
            }
        } {
            // {path}.r.*
            new_uri_path = Some(rendered_page_uri_path);
        }

        if let Some(new_uri_path) = new_uri_path {
            tracing::debug!("rewriting: {} to {}", original_uri_path, new_uri_path);
            let original_uri_path = original_uri_path.into_owned().into();
            if let Err(_error) = request.set_uri_path(&new_uri_path) {
                return request.with_deferred_error("set URI path".into());
            }
            return request.with_deferred_rewrite_from(original_uri_path);
        }

        request
    }
}
