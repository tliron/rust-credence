use super::super::parse::*;

use {bytestring::*, compris::resolve::*};

//
// Redirect
//

/// Redirect.
#[derive(Clone, Debug, Default, Resolve)]
pub struct Redirect {
    /// Regex.
    ///
    /// See [implementation syntax](https://docs.rs/regex/latest/regex/#syntax).
    #[resolve(required)]
    pub regex: Option<ResolveRegex>,

    /// To.
    ///
    /// See [implementation syntax](https://docs.rs/regex/latest/regex/struct.Captures.html#method.expand).
    #[resolve(required, key = "to")]
    pub to_: ByteString,
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
