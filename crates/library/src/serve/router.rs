use super::super::{configuration::*, middleware::*};

use {
    ::axum::{extract::*, http::*, middleware::*, routing::*},
    kutil_http::{
        axum::*,
        cache::{axum::*, *},
    },
    tower_http::{limit::*, services::*, timeout::*, trace::*},
};

/// Create a Credence site router.
pub fn new_router<CacheT, CacheKeyT>(
    shutdown: &Shutdown,
    cache: &CacheT,
    configuration: &CredenceConfiguration,
) -> Router
where
    CacheT: Cache<CacheKeyT>,
    CacheKeyT: CacheKey,
{
    let admin_router = Router::new()
        .route("/shutdown", post(shutdown_handler))
        .with_state(shutdown.clone())
        .route("/reset-cache", post(reset_cache_handler::<CacheT, _>))
        .with_state(cache.clone())
        .route("/status/{status_code}", get(status_code));

    let router = Router::new()
        .fallback_service(ServeDir::new(&configuration.paths.assets).append_index_html_on_directories(false))
        .nest("/admin", admin_router)
        .layer(from_fn_with_state(RenderMiddleware::new(configuration.clone()), RenderMiddleware::function))
        .layer(configuration.caching_layer(cache.clone()))
        .layer(from_fn_with_state(CatchMiddleware::new(configuration.paths.status.clone()), CatchMiddleware::function));

    // Request rewriting cannot happen in the handling router
    // https://docs.rs/axum/latest/axum/middleware/index.html#rewriting-request-uri-in-middleware

    let router = Router::new()
        .merge(router)
        .layer(map_request_with_state(FacadeMiddleware::new(configuration.clone()), FacadeMiddleware::function))
        .layer(RequestBodyLimitLayer::new(configuration.requests.max_body_size.value.into()))
        .layer(TimeoutLayer::new(configuration.requests.max_duration.value.into()))
        .layer(TraceLayer::new_for_http());

    router
}

async fn status_code(Path(status_code): Path<u16>) -> StatusCode {
    tracing::debug!("status code: {}", status_code);
    StatusCode::from_u16(status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
}
