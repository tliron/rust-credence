use {chrono::*, compris::resolve::*, dateparser::*};

//
// ParseDateTime
//

/// [ParseStr] for [DateTime].
///
/// See [dateparser](https://docs.rs/dateparser/latest/dateparser/#accepted-date-formats).
#[derive(Clone, Default, Debug)]
pub struct ParseDateTime {}

impl ParseStr<DateTime<Utc>> for ParseDateTime {
    fn parse(representation: &str) -> Option<DateTime<Utc>> {
        parse(representation).ok()
    }
}
