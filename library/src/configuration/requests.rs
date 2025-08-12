use super::super::resolve::*;

use {
    compris::resolve::*,
    kutil::{cli::depict::*, std::metric::*},
    std::time::*,
};

//
// RequestsConfiguration
//

/// Requests configuration.
#[derive(Clone, Debug, Depict, Resolve)]
pub struct RequestsConfiguration {
    /// Maximum body size.
    #[resolve(key = "max-body-size")]
    #[depict(as(display), style(symbol))]
    pub max_body_size: ResolveByteCount,

    /// Maximum duration.
    #[resolve(key = "max-duration")]
    #[depict(as(custom(resolve_duration_to_string)), style(symbol))]
    pub max_duration: ResolveDuration,
}

impl Default for RequestsConfiguration {
    fn default() -> Self {
        Self { max_body_size: ByteCount::from_kibibytes(4).into(), max_duration: Duration::from_secs(10).into() }
    }
}
