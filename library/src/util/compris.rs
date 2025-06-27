use {::compris::normal::*, kutil_http::*, std::cmp::*};

/// Compare [Variant] as lowercase.
pub fn cmp_variant_lowercase<AnnotatedT>(a: &Variant<AnnotatedT>, b: &Variant<AnnotatedT>) -> Ordering {
    if let Variant::Text(a) = a
        && let Variant::Text(b) = b
    {
        return a.inner.to_lowercase().cmp(&b.inner.to_lowercase());
    }

    a.cmp(b)
}

/// [QueryMap] to [Variant].
pub fn query_map_to_variant<AnnotatedT>(query: &QueryMap) -> Variant<AnnotatedT>
where
    AnnotatedT: Default,
{
    query
        .into_iter()
        .map(|(key, values)| {
            (key.clone().into(), values.into_iter().map(|value| Variant::from(value.clone())).collect())
        })
        .collect()
}
