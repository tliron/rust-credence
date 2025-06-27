use {
    compris::{annotate::*, normal::*},
    kutil_std::collections::*,
};

//
// ExtraColumns
//

/// Extra columns.
pub type ExtraColumns<'own> = FastHashMap<&'own str, RefTraversal<'own, WithAnnotations>>;

/// Flatten extra columns.
pub fn flatten_columns(columns: Option<&Map<WithAnnotations>>) -> ExtraColumns<'_> {
    let mut flat_columns = FastHashMap::<&str, _>::default();

    if let Some(columns) = columns {
        for (key, value) in columns {
            if let Variant::Text(key) = key
                && let Some(value_path) = to_ref_traversal(value)
            {
                flat_columns.insert(key.into(), value_path);
            }
        }
    }

    flat_columns
}
