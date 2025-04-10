use {compris::resolve::*, duration_str::*, std::time::*};

//
// ParseDuration
//

/// [ParseStr] for [Duration].
///
/// See [duration-str](https://docs.rs/duration-str/latest/duration_str/).
#[derive(Clone, Default, Debug)]
pub struct ParseDuration {}

impl ParseStr<Duration> for ParseDuration {
    fn parse(representation: &str) -> Option<Duration> {
        parse(representation).ok()
    }
}
