use {compris::normal::*, std::cmp::*};

/// Compare lowercase.
pub fn cmp_lowercase(a: &Value, b: &Value) -> Ordering {
    if let Value::Text(a) = a {
        if let Value::Text(b) = b {
            return a.value.to_lowercase().cmp(&b.value.to_lowercase());
        }
    }

    a.cmp(b)
}
