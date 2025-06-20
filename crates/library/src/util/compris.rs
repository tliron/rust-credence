use {::compris::normal::*, kutil_http::*, std::cmp::*};

/// Compare [Value] as lowercase.
pub fn cmp_value_lowercase(a: &Value, b: &Value) -> Ordering {
    if let Value::Text(a) = a {
        if let Value::Text(b) = b {
            return a.value.to_lowercase().cmp(&b.value.to_lowercase());
        }
    }

    a.cmp(b)
}

/// [QueryMap] to [Value].
pub fn query_map_to_value(query: &QueryMap) -> Value {
    query
        .into_iter()
        .map(|(key, values)| (key.clone().into(), values.into_iter().map(|value| Value::from(value.clone())).collect()))
        .collect()
}
