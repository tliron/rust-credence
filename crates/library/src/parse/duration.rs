use {compris::resolve::*, duration_str::*, std::time::*};

/// [Duration] that implements [Resolve].
pub type ResolveDuration = ResolveParseStr<Duration, ParseDuration>;

//
// ParseDuration
//

/// [ParseStr] for [Duration].
///
/// See [duration-str](https://docs.rs/duration-str/latest/duration_str/).
#[derive(Clone, Default, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ParseDuration {}

impl ParseStr<Duration> for ParseDuration {
    fn parse(representation: &str) -> Option<Duration> {
        parse(representation).ok()
    }
}
