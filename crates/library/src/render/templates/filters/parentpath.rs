use super::super::super::super::util::*;

use std::borrow::*;

/// `parentpath` filter for MiniJinja.
///
/// Returns the parent path.
pub fn parentpath_filter(value: Cow<'_, str>) -> String {
    uri_path_parent(value.as_ref()).into()
}
