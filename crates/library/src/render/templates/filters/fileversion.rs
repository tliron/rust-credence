use {
    kutil_std::fs::*,
    minijinja::*,
    std::{borrow::*, path::*},
};

/// `version` filter for MiniJinja.
///
/// The value is a path relative to the `ASSETS_PATH` global, which must be set.
///
/// The returned value is intended to be opaque, however it should be unique for every file
/// modification. It will be 0 if there is any error.
pub fn fileversion_filter(state: &State, value: Cow<'_, str>) -> u64 {
    if let Some(assets_path) = state.lookup("ASSETS_PATH") {
        if let Some(assets_path) = assets_path.as_str() {
            let assets_path: PathBuf = assets_path.into();
            let path = assets_path.join(value.as_ref());
            if let Ok(identifier) = file_modification_identifier(path) {
                return identifier;
            }
        }
    }

    0
}
