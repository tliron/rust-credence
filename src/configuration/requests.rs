use super::super::parse::*;

use {compris::resolve::*, kutil_std::metric::*, std::time::*};

//
// RequestsConfiguration
//

/// Requests configuration.
#[derive(Clone, Debug, Resolve)]
pub struct RequestsConfiguration {
    /// Maximum body size.
    #[resolve(key = "max-body-size")]
    pub max_body_size: ResolveFromStr<ByteCount>,

    /// Maximum duration.
    #[resolve(key = "max-duration")]
    pub max_duration: ResolveParseStr<Duration, ParseDuration>,
}

impl Default for RequestsConfiguration {
    fn default() -> Self {
        Self {
            max_body_size: ByteCount::from_kibibytes(4).into(),
            max_duration: Duration::from_secs(5).into(),
        }
    }
}
