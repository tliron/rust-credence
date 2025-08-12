use super::super::{configuration::*, middleware::*};

use {
    ::axum::{
        extract::{Request, *},
        http::{header::*, *},
        middleware::*,
        response::*,
        routing::*,
    },
    kutil::{
        http::{
            axum::*,
            cache::{axum::*, *},
        },
        std::immutable::*,
    },
    tower_http::{limit::*, services::*, timeout::*, trace::*},
};

/// Create a Credence site router.
pub fn new_site_router<CacheT>(shutdown: &Shutdown, cache: &CacheT, configuration: &CredenceConfiguration) -> Router
where
    CacheT: Cache<CommonCacheKey>,
{
    let admin_router = Router::default()
        .route("/about", get(about_handler))
        .route("/shutdown", post(shutdown_handler))
        .with_state(shutdown.clone())
        .route("/reset-cache", post(reset_cache_handler::<CacheT, _>))
        .with_state(cache.clone())
        .route("/status/{status_code}", get(status_code_handler));

    let router = Router::default()
        .fallback_service(ServeDir::new(&configuration.files.assets).append_index_html_on_directories(false))
        .nest("/admin", admin_router)
        .layer(from_fn_with_state(RenderMiddleware::new(configuration.clone()), RenderMiddleware::function))
        .layer(configuration.caching_layer(cache.clone()))
        .layer(from_fn_with_state(CatchMiddleware::new(configuration.files.status.clone()), CatchMiddleware::function));

    // Request rewriting cannot happen in the handling router
    // https://docs.rs/axum/latest/axum/middleware/index.html#rewriting-request-uri-in-middleware

    let router = Router::default()
        .merge(router)
        .layer(map_request_with_state(FacadeMiddleware::new(configuration.clone()), FacadeMiddleware::function))
        .layer(RequestBodyLimitLayer::new(configuration.requests.max_body_size.inner.into()))
        .layer(TimeoutLayer::new(configuration.requests.max_duration.inner.into()))
        .layer(TraceLayer::new_for_http());

    router
}

/// Create a Credence redirecting router.
pub fn new_redirecting_router(tls: bool, host: ByteString, tcp_port: u16) -> Router {
    let scheme = if tls { "https://" } else { "http://" };

    let tcp_port = match tcp_port {
        80 | 443 => Default::default(),
        _ => format!(":{}", tcp_port),
    };

    Router::default().fallback(async move |request: Request| {
        let path_and_query = request
            .uri()
            .path_and_query()
            .map(|path_and_query| path_and_query.to_string())
            .unwrap_or_else(|| "/".into());

        let uri = format!("{}{}{}{}", scheme, host, tcp_port, path_and_query);

        (StatusCode::MOVED_PERMANENTLY, [(LOCATION, uri)]).into_response()
    })
}

async fn status_code_handler(Path(status_code): Path<u16>) -> StatusCode {
    tracing::debug!("status code: {}", status_code);
    StatusCode::from_u16(status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
}

build_info::build_info!(fn build_info);

async fn about_handler() -> impl IntoResponse {
    let build_info = build_info();
    let mut about = String::default();

    about += &format!("credence-lib: {}\n", env!("CARGO_PKG_VERSION"));

    if let Some(version_control) = &build_info.version_control
        && let Some(git) = version_control.git()
    {
        about += "\n";
        about += &format!("git-commit-id: {}\n", git.commit_id);
        about += &format!("git-commit-timestamp: {}\n", git.commit_timestamp);
        if let Some(branch) = &git.branch {
            about += &format!("git-commit-branch: {}\n", branch);
        }
        if !git.tags.is_empty() {
            about += &format!("git-commit-tags: {}\n", git.tags.join(","));
        }
        about += &format!("git-dirty: {}]\n", git.dirty);
    }

    about += "\n";
    about += &format!("binary-cpu: {}\n", build_info.target.cpu.arch);
    about += &format!("binary-cpu-bits: {}]\n", build_info.target.cpu.pointer_width);
    about += &format!("binary-cpu-endianness: {}\n", build_info.target.cpu.endianness.to_string().to_lowercase());
    if !build_info.target.cpu.features.is_empty() {
        about += &format!("binary-cpu-features: {}\n", build_info.target.cpu.features.join(","));
    }
    about += &format!("binary-os: {}\n", build_info.target.os);
    about += &format!("binary-architecture: {}\n", build_info.target.triple);

    about += "\n";
    about += &format!("compilation-timestamp: {}\n", build_info.timestamp);
    about += &format!("compilation-profile: {}\n", build_info.profile);
    about += &format!("compilation-optimization-level: {}\n", build_info.optimization_level);

    about += "\n";
    about += "compiler: rustc\n";
    about += &format!("compiler-version: {}\n", build_info.compiler.version);
    about += &format!("compiler-channel: {}\n", build_info.compiler.channel.to_string().to_lowercase());
    if let Some(commit_id) = &build_info.compiler.commit_id {
        about += &format!("compiler-git-commit-id: {}\n", commit_id);
    }
    if let Some(commit_date) = &build_info.compiler.commit_date {
        about += &format!("compiler-git-commit-date: {}\n", commit_date);
    }

    ([("content-type", "text/plain")], about)
}
