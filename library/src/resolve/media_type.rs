use {compris::resolve::*, kutil_http::*};

/// [MediaType] that implements [Resolve].
pub type ResolveMediaType = ResolveFromStr<MediaType>;
