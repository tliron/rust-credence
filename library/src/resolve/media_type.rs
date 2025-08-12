use {compris::resolve::*, kutil::http::*};

/// [MediaType] that implements [Resolve].
pub type ResolveMediaType = ResolveFromStr<MediaType>;
