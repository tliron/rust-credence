use {compris::resolve::*, regex::*};

//
// Redirect
//

/// Redirect.
#[derive(Clone, Debug, Default, Resolve)]
pub struct Redirect {
    /// Regex.
    #[resolve(required)]
    pub regex: Option<ResolveFromStr<Regex>>,

    /// To.
    #[resolve(required, key = "to")]
    pub to_: String,
}

impl Redirect {
    /// If the URI is redirected returns the redirected URI.
    pub fn redirect(&self, uri_path: &str) -> Option<String> {
        if let Some(regex) = &self.regex {
            if let Some(captures) = regex.value.captures(uri_path) {
                let mut uri_path = String::new();
                captures.expand(&self.to_, &mut uri_path);
                return Some(uri_path);
            }
        }

        None
    }
}
