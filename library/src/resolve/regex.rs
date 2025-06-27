use {compris::resolve::*, regex::*};

/// [Regex] that implements [Resolve].
pub type ResolveRegex = ResolveFromStr<Regex>;
