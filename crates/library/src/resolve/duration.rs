use {
    compris::resolve::*,
    duration_str::*,
    kutil_std::string::*,
    std::{io::*, result::Result, time::*},
};

/// [Duration] that implements [Resolve].
pub type ResolveDuration = ResolveParseStr<Duration, ParseDuration>;

/// [ResolveDuration] to string.
pub fn resolve_duration_to_string(duration: &ResolveDuration) -> Result<String, Error> {
    Ok(duration.value.human_format())
}

//
// ParseDuration
//

/// [ParseStr] for [Duration].
///
/// See [duration-str](https://docs.rs/duration-str/latest/duration_str/).
#[derive(Clone, Default, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ParseDuration {}

impl ParseStr<Duration> for ParseDuration {
    fn parse(representation: &str) -> Result<Duration, ParseError> {
        Ok(parse(representation)?)
    }
}
