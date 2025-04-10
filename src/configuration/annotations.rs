use super::constants::*;

use compris::{resolve::*, *};

//
// AnnotationsConfiguration
//

/// Annotations configuration.
#[derive(Clone, Debug, Resolve)]
pub struct AnnotationsConfiguration {
    /// Start delimiter.
    #[resolve(key = "start-delimiter")]
    pub start_delimiter: String,

    /// End delimiter.
    #[resolve(key = "end-delimiter")]
    pub end_delimiter: String,

    /// Default format.
    #[resolve(key = "default-format")]
    pub default_format: ResolveFromStr<Format>,
}

impl Default for AnnotationsConfiguration {
    fn default() -> Self {
        Self {
            start_delimiter: DEFAULT_ANNOTATIONS_DELIMITER.into(),
            end_delimiter: DEFAULT_ANNOTATIONS_DELIMITER.into(),
            default_format: Format::default().into(),
        }
    }
}
