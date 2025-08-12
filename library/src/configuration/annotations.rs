use {
    compris::{resolve::*, *},
    kutil::{cli::depict::*, std::immutable::*},
};

//
// AnnotationsConfiguration
//

/// Annotations configuration.
#[derive(Clone, Debug, Depict, Resolve)]
pub struct AnnotationsConfiguration {
    /// Start delimiter.
    #[resolve(key = "start-delimiter")]
    #[depict(style(string))]
    pub start_delimiter: ByteString,

    /// End delimiter.
    #[resolve(key = "end-delimiter")]
    #[depict(style(string))]
    pub end_delimiter: ByteString,

    /// Default format.
    #[resolve(key = "default-format")]
    #[depict(as(display), style(symbol))]
    pub default_format: ResolveFromStr<Format>,
}

impl Default for AnnotationsConfiguration {
    fn default() -> Self {
        Self { start_delimiter: "```".into(), end_delimiter: "```".into(), default_format: Default::default() }
    }
}
