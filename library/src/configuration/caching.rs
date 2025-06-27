use super::super::resolve::*;

use {
    compris::resolve::*,
    kutil_cli::debug::*,
    kutil_http::cache::{implementation::moka::*, *},
    kutil_std::metric::*,
    moka::future::Cache,
    std::time::*,
};

//
// CachingConfiguration
//

/// Caching configuration.
#[derive(Clone, Debug, Debuggable, Resolve)]
pub struct CachingConfiguration {
    /// Default.
    #[resolve]
    #[debuggable(style(symbol))]
    pub default: bool,

    /// Capacity.
    #[resolve]
    #[debuggable(as(display), style(symbol))]
    pub capacity: ResolveByteCount,

    /// Cache entry duration.
    #[resolve(key = "duration")]
    #[debuggable(as(custom(resolve_duration_to_string)), style(symbol))]
    pub duration: ResolveDuration,

    /// Minimum cacheable body size.
    #[resolve(key = "min-body-size")]
    #[debuggable(as(display), style(symbol))]
    pub min_body_size: ResolveByteCount,

    /// Maximum cacheable body size.
    #[resolve(key = "max-body-size")]
    #[debuggable(as(display), style(symbol))]
    pub max_body_size: ResolveByteCount,
}

impl CachingConfiguration {
    /// Cache.
    pub fn cache(&self) -> MokaCacheImplementation {
        let cache = Cache::<CommonCacheKey, _>::builder()
            .for_http_response()
            .max_capacity(self.capacity.inner.into())
            .time_to_live(self.duration.inner.into())
            .eviction_listener(|key, _value, cause| {
                tracing::debug!("evict ({:?}): {}", cause, key);
            })
            .build();

        MokaCacheImplementation::new(cache)
    }
}

impl Default for CachingConfiguration {
    fn default() -> Self {
        Self {
            default: true,
            capacity: ByteCount::from_gibibytes(1).into(),
            duration: Duration::from_secs(5).into(),
            min_body_size: Default::default(),
            max_body_size: ByteCount::from_mebibytes(10).into(),
        }
    }
}
