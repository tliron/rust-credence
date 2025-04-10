use super::super::parse::*;

use {
    compris::resolve::*,
    kutil_http::cache::{implementation::moka::*, *},
    kutil_std::metric::*,
    moka::future::Cache,
    std::time::*,
};

//
// CachingConfiguration
//

/// Caching configuration.
#[derive(Clone, Debug, Resolve)]
pub struct CachingConfiguration {
    /// Default.
    #[resolve]
    pub default: bool,

    /// Capacity.
    #[resolve]
    pub capacity: ResolveByteCount,

    /// Time-to-idle.
    #[resolve(key = "time-to-idle")]
    pub time_to_idle: ResolveDuration,

    /// Minimum cacheable body size.
    #[resolve(key = "min-body-size")]
    pub min_body_size: ResolveByteCount,

    /// Maximum cacheable body size.
    #[resolve(key = "max-body-size")]
    pub max_body_size: ResolveByteCount,
}

impl CachingConfiguration {
    /// Cache.
    pub fn cache(&self) -> MokaCacheImplementation {
        let cache = Cache::<CommonCacheKey, _>::builder()
            .for_http_response()
            .max_capacity(self.capacity.value.into())
            .time_to_idle(self.time_to_idle.value.into())
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
            time_to_idle: Duration::from_secs(5).into(),
            min_body_size: ByteCount::default().into(),
            max_body_size: ByteCount::from_mebibytes(10).into(),
        }
    }
}
