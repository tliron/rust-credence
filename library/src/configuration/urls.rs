use super::{
    super::{resolve::*, util::*},
    protect::*,
    redirect::*,
};

use {axum::http::*, compris::resolve::*, kutil::cli::depict::*, regex::*};

//
// UrlsConfiguration
//

/// URLs configuration.
#[derive(Clone, Debug, Depict, Resolve)]
pub struct UrlsConfiguration {
    /// Hide.
    #[resolve(key = "hide")]
    #[depict(iter(item), as(display), style(string))]
    pub hide: Vec<ResolveRegex>,

    /// Redirect trailing slashes.
    #[resolve(key = "redirect-trailing-slashes")]
    #[depict(style(symbol))]
    pub redirect_trailing_slashes: bool,

    /// Redirect.
    #[resolve]
    #[depict(iter(item), as(depict))]
    pub redirect: Vec<Redirect>,

    /// Protect.
    #[resolve]
    #[depict(iter(item), as(depict))]
    pub protect: Vec<Protect>,
}

impl UrlsConfiguration {
    /// If the URI path is redirected returns the redirected URI.
    pub fn redirect(&self, uri_path: &str) -> Option<(String, StatusCode)> {
        match self.has_trailing_slashes(uri_path) {
            Some(uri_path) => Some((uri_path.into(), StatusCode::MOVED_PERMANENTLY)),

            None => {
                for redirect in &self.redirect {
                    if let Some(uri_path) = redirect.redirect(uri_path) {
                        return Some(uri_path);
                    }
                }

                return None;
            }
        }
    }

    /// If `redirect_trailing_slashes` is true and the URI has trailing slashes returns it without
    /// them.
    pub fn has_trailing_slashes<'path>(&self, uri_path: &'path str) -> Option<&'path str> {
        if self.redirect_trailing_slashes && (uri_path.len() > 1) && uri_path.ends_with(PATH_SEPARATOR) {
            Some(uri_path.trim_end_matches(PATH_SEPARATOR))
        } else {
            None
        }
    }

    /// If the URI is protected returns the [Protect].
    pub fn protect(&self, uri_path: &str) -> Option<&Protect> {
        for protect in &self.protect {
            if protect.protect(uri_path) {
                return Some(protect);
            }
        }

        None
    }
}

impl Default for UrlsConfiguration {
    fn default() -> Self {
        Self {
            hide: vec![Regex::new(r"\.html$").expect("regex").into()],
            redirect_trailing_slashes: true,
            redirect: Default::default(),
            protect: Default::default(),
        }
    }
}
