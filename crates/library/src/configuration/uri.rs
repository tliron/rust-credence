use super::{
    super::{resolve::*, util::*},
    constants::*,
    protection::*,
    redirect::*,
};

use {axum::http::*, compris::resolve::*, kutil_cli::debug::*, regex::*};

//
// UriConfiguration
//

/// URI configuration.
#[derive(Clone, Debug, Debuggable, Resolve)]
pub struct UriConfiguration {
    /// Hide.
    #[resolve(key = "hide")]
    #[debuggable(iter(item), as(display), style(string))]
    pub hide: Vec<ResolveRegex>,

    /// Redirect trailing slashes.
    #[resolve(key = "redirect-trailing-slashes")]
    #[debuggable(style(bare))]
    pub redirect_trailing_slashes: bool,

    /// Redirect.
    #[resolve]
    #[debuggable(iter(item), as(debuggable))]
    pub redirect: Vec<Redirect>,

    /// Hide suffixes.
    #[resolve]
    #[debuggable(iter(item), as(debuggable))]
    pub protect: Vec<Protection>,
}

impl UriConfiguration {
    /// If the URI is redirected returns the redirected URI.
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

    /// If the URI is protected returns the [Protection].
    pub fn protection(&self, uri_path: &str) -> Option<&Protection> {
        for protection in &self.protect {
            if protection.protected(uri_path) {
                return Some(protection);
            }
        }

        None
    }
}

impl Default for UriConfiguration {
    fn default() -> Self {
        Self {
            hide: vec![Regex::new(HTML_SUFFIX_REGEX).expect("regex").into()],
            redirect_trailing_slashes: true,
            redirect: Vec::default(),
            protect: Vec::default(),
        }
    }
}
