use super::{super::super::util::*, columns::*};

use {
    compris::{annotate::*, normal::*},
    std::cmp::*,
};

//
// CatalogAnnotation
//

/// Catalog annotation.
pub struct CatalogAnnotation<'own> {
    /// Sort column.
    pub sort_column: Option<RefTraversal<'own, WithAnnotations>>,

    /// Sort ascending.
    pub sort_ascending: bool,

    /// Extra columns.
    pub extra_columns: ExtraColumns<'own>,
}

impl<'own> CatalogAnnotation<'own> {
    /// Constructor.
    pub fn new(
        sort_column: Option<RefTraversal<'own, WithAnnotations>>,
        sort_ascending: bool,
        extra_columns: ExtraColumns<'own>,
    ) -> Self {
        Self { sort_column, sort_ascending, extra_columns }
    }

    /// Resolve.
    pub fn resolve(variant: &'own Variant<WithAnnotations>) -> Self {
        let extra_columns = flatten_columns(
            traverse!(variant, "columns").and_then(
                |columns| {
                    if let Variant::Map(map) = columns { Some(map) } else { None }
                },
            ),
        );

        let (sort_column, sort_ascending) = traverse!(variant, "sort")
            .and_then(|sort| match sort {
                Variant::Map(map) => Some((
                    map.inner.get(&"column".into()).and_then(to_ref_traversal),
                    map.inner
                        .get(&"ascending".into())
                        .map(|ascending| match ascending {
                            Variant::Boolean(ascending) => ascending.inner,
                            _ => true,
                        })
                        .unwrap_or(true),
                )),

                Variant::List(_) | Variant::Text(_) => Some((to_ref_traversal(sort), true)),

                _ => None,
            })
            .unwrap_or((None, true));

        Self::new(sort_column, sort_ascending, extra_columns)
    }

    /// Sort rows by column.
    pub fn sort(self, rows: &mut Vec<Variant<WithAnnotations>>) {
        let ascending = self.sort_ascending;
        let sort_column = self.into_sort_column();
        rows.sort_by(|row1, row2| cmp_rows_by_column(row1, row2, &sort_column, ascending));
    }

    /// Into sort column (defaults to "title").
    fn into_sort_column(self) -> Traversal<WithAnnotations> {
        self.sort_column.map(to_traversal).unwrap_or_else(|| vec!["title".into()])
    }
}

fn cmp_rows_by_column<AnnotatedT>(
    row1: &Variant<AnnotatedT>,
    row2: &Variant<AnnotatedT>,
    column_path: &Traversal<AnnotatedT>,
    ascending: bool,
) -> Ordering {
    if let Some(a) = row1.traverse(column_path.iter())
        && let Some(b) = row2.traverse(column_path.iter())
    {
        let ordering = cmp_variant_lowercase(a, b);
        if ascending { ordering } else { ordering.reverse() }
    } else {
        Ordering::Less
    }
}
