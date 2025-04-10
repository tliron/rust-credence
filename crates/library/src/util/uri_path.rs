use super::constants::*;

use std::str::*;

/// URI path file name.
pub fn uri_path_file_name(uri_path: &str) -> &str {
    if let Some(last_slash) = uri_path.rfind(PATH_SEPARATOR) { &uri_path[last_slash + 1..] } else { uri_path }
}

/// URI path segments.
pub fn uri_path_segments(uri_path: &str) -> Split<'_, char> {
    uri_path.split(PATH_SEPARATOR)
}

/// Whether the URI path has a hidden segment.
pub fn uri_path_has_hidden_segment(uri_path: &str) -> bool {
    for segment in uri_path_segments(uri_path) {
        if segment.starts_with(HIDDEN_PATH_PREFIX) {
            return true;
        }
    }

    false
}

/// Join URI paths.
pub fn uri_path_join(uri_path_1: &str, uri_path_2: &str) -> String {
    uri_path_1.trim_end_matches(PATH_SEPARATOR).to_string() + PATH_SEPARATOR_STRING + uri_path_2
}

/// Up one segment for the URI path.
///
/// Will return [None](Option::None) if at root.
pub fn uri_path_parent(uri_path: &str) -> &str {
    let last_slash = uri_path.rfind(PATH_SEPARATOR).unwrap_or(1);
    &uri_path[..last_slash]
}
