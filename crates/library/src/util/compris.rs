use {::compris::normal::*, kutil_http::*, std::cmp::*};

/// Compare [Value] as lowercase.
pub fn cmp_value_lowercase<AnnotatedT>(a: &Value<AnnotatedT>, b: &Value<AnnotatedT>) -> Ordering {
    if let Value::Text(a) = a
        && let Value::Text(b) = b
    {
        return a.inner.to_lowercase().cmp(&b.inner.to_lowercase());
    }

    a.cmp(b)
}

/// [QueryMap] to [Value].
pub fn query_map_to_value<AnnotatedT>(query: &QueryMap) -> Value<AnnotatedT>
where
    AnnotatedT: Default,
{
    query
        .into_iter()
        .map(|(key, values)| (key.clone().into(), values.into_iter().map(|value| Value::from(value.clone())).collect()))
        .collect()
}
