use super::constants::*;

use std::path::*;

/// Filename with extension and without extension.
pub fn file_name<PathT>(path: PathT) -> (String, String)
where
    PathT: AsRef<Path>,
{
    let path = path.as_ref().to_string_lossy();

    let mut filename = path.as_ref();
    if let Some(last_slash) = filename.rfind(MAIN_SEPARATOR) {
        filename = &filename[last_slash + 1..];
    }

    let mut filename_without_extension = filename;
    if let Some(first_dot) = filename_without_extension.find(EXTENSION_SEPARATOR) {
        filename_without_extension = &filename_without_extension[..first_dot];
    }

    (filename.into(), filename_without_extension.into())
}
