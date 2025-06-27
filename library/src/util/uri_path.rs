use super::constants::*;

use std::str::*;

/// URI path segments.
pub fn uri_path_segments(uri_path: &str) -> Split<'_, char> {
    uri_path.split(PATH_SEPARATOR)
}

/// The last segment in the URI path.
pub fn uri_path_last_segment(uri_path: &str) -> &str {
    match uri_path.rfind(PATH_SEPARATOR) {
        Some(last_slash) => &uri_path[last_slash + 1..],
        None => uri_path,
    }
}

/// Whether the URI path has any segment that begins with ".".
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
pub fn uri_path_parent(uri_path: &str) -> &str {
    let mut last_slash = uri_path.rfind(PATH_SEPARATOR).unwrap_or(1);
    if last_slash == 0 {
        last_slash = 1
    }
    &uri_path[..last_slash]
}
