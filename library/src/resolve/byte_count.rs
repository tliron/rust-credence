use {compris::resolve::*, kutil::std::metric::*};

/// [ByteCount] that implements [Resolve].
pub type ResolveByteCount = ResolveFromStr<ByteCount>;
