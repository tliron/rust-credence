use {axum::http::*, compris::resolve::*};

/// [StatusCode] that implements [Resolve].
pub type ResolveStatusCode = ResolveTryFrom<StatusCode, u16>;
