use {
    compris::{resolve::*, *},
    kutil_cli::debug::*,
    kutil_std::zerocopy::*,
};

//
// AnnotationsConfiguration
//

/// Annotations configuration.
#[derive(Clone, Debug, Debuggable, Resolve)]
pub struct AnnotationsConfiguration {
    /// Start delimiter.
    #[resolve(key = "start-delimiter")]
    #[debuggable(style(string))]
    pub start_delimiter: ByteString,

    /// End delimiter.
    #[resolve(key = "end-delimiter")]
    #[debuggable(style(string))]
    pub end_delimiter: ByteString,

    /// Default format.
    #[resolve(key = "default-format")]
    #[debuggable(as(display), style(symbol))]
    pub default_format: ResolveFromStr<Format>,
}

impl Default for AnnotationsConfiguration {
    fn default() -> Self {
        Self { start_delimiter: "```".into(), end_delimiter: "```".into(), default_format: Default::default() }
    }
}
