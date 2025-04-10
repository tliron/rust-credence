use super::{cli::*, configuration::*, middleware::*};

use {
    ::axum::{middleware::*, routing::*},
    kutil_http::cache::{axum::*, *},
    tower_http::{limit::*, services::*, timeout::*, trace::*},
};

impl CLI {
    /// Router.
    pub fn router<CacheT, CacheKeyT>(
        &self,
        cache: CacheT,
        configuration: &ServerConfiguration,
    ) -> Router
    where
        CacheT: Cache<CacheKeyT>,
        CacheKeyT: CacheKey,
    {
        let admin_router = Router::new()
            .route("/reset-cache", post(reset_cache::<CacheT, _>))
            .with_state(cache.clone());

        let router = Router::new()
            .fallback_service(ServeDir::new(&configuration.paths.assets))
            .nest("/admin", admin_router)
            .layer(from_fn_with_state(
                RenderMiddleware::new(configuration.clone()),
                RenderMiddleware::function,
            ))
            .layer(configuration.caching_layer(cache))
            .layer(from_fn_with_state(
                StatusMiddleware::new(configuration.paths.status.clone()),
                StatusMiddleware::function,
            ));

        // Request rewriting cannot happen in the handling router
        // https://docs.rs/axum/latest/axum/middleware/index.html#rewriting-request-uri-in-middleware

        let router = Router::new()
            .merge(router)
            .layer(map_request_with_state(
                UriMiddleware::new(configuration.clone()),
                UriMiddleware::function,
            ))
            .layer(RequestBodyLimitLayer::new(
                configuration.requests.max_body_size.value.into(),
            ))
            .layer(TimeoutLayer::new(
                configuration.requests.max_duration.value.into(),
            ))
            .layer(TraceLayer::new_for_http());

        router
    }
}
