use super::{super::super::util::*, columns::*};

use {
    compris::{normal::*, *},
    std::cmp::*,
};

//
// CatalogAnnotation
//

/// Catalog annotation.
pub struct CatalogAnnotation<'own> {
    /// Sort column.
    pub sort_column: Option<RefValuePath<'own>>,

    /// Sort ascending.
    pub sort_ascending: bool,

    /// Extra columns.
    pub extra_columns: ExtraColumns<'own>,
}

impl<'own> CatalogAnnotation<'own> {
    /// Constructor.
    pub fn new(
        sort_column: Option<RefValuePath<'own>>,
        sort_ascending: bool,
        extra_columns: ExtraColumns<'own>,
    ) -> Self {
        Self { sort_column, sort_ascending, extra_columns }
    }

    /// Resolve.
    pub fn resolve(value: &'own Value) -> Self {
        let extra_columns = flatten_columns(
            traverse!(value, "columns").and_then(
                |columns| {
                    if let Value::Map(map) = columns { Some(map) } else { None }
                },
            ),
        );

        let (sort_column, sort_ascending) = traverse!(value, "sort")
            .and_then(|sort| match sort {
                Value::Map(map) => Some((
                    map.value.get(&"column".into()).and_then(to_ref_value_path),
                    map.value
                        .get(&"ascending".into())
                        .map(|ascending| match ascending {
                            Value::Boolean(ascending) => ascending.value,
                            _ => true,
                        })
                        .unwrap_or(true),
                )),

                Value::List(_) | Value::Text(_) => Some((to_ref_value_path(sort), true)),

                _ => None,
            })
            .unwrap_or((None, true));

        Self::new(sort_column, sort_ascending, extra_columns)
    }

    /// Sort rows by column.
    pub fn sort(self, rows: &mut Vec<Value>) {
        let ascending = self.sort_ascending;
        let sort_column = self.into_sort_column();
        rows.sort_by(|row1, row2| cmp_rows_by_column(row1, row2, &sort_column, ascending));
    }

    /// Into sort column (defaults to "title").
    fn into_sort_column(self) -> ValuePath {
        self.sort_column.map(to_value_path).unwrap_or_else(|| vec!["title".into()])
    }
}

fn cmp_rows_by_column(row1: &Value, row2: &Value, column_path: &ValuePath, ascending: bool) -> Ordering {
    if let Some(a) = row1.traverse(column_path.iter()) {
        if let Some(b) = row2.traverse(column_path.iter()) {
            let ordering = cmp_value_lowercase(a, b);
            return if ascending { ordering } else { ordering.reverse() };
        }
    }

    Ordering::Less
}
