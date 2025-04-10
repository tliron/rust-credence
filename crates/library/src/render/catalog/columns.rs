use {compris::normal::*, kutil_std::collections::*};

//
// ExtraColumns
//

/// Extra columns.
pub type ExtraColumns<'own> = FastHashMap<&'own str, RefValuePath<'own>>;

/// Flatten extra columns.
pub fn flatten_columns(columns: Option<&Map>) -> ExtraColumns<'_> {
    let mut flat_columns = FastHashMap::<&str, _>::new();

    if let Some(columns) = columns {
        for (key, value) in columns {
            if let Value::Text(key) = key {
                if let Some(value_path) = to_ref_value_path(value) {
                    flat_columns.insert(&key.value, value_path);
                }
            }
        }
    }

    flat_columns
}
