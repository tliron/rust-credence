use {compris::resolve::*, kutil_std::metric::*};

/// [ByteCount] that implements [Resolve].
pub type ResolveByteCount = ResolveFromStr<ByteCount>;
