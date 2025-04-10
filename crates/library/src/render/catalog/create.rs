use super::{
    super::{
        super::{configuration::*, util::*},
        constants::*,
        rendered_page::*,
    },
    annotation::*,
};

use {
    ::axum::http::*,
    compris::normal::*,
    kutil_http::*,
    std::{path::*, result::Result},
    tokio::fs::*,
};

/// Create catalog.
pub async fn create_catalog<'annotation, PathT>(
    annotation: CatalogAnnotation<'annotation>,
    uri_path: &str,
    directory: PathT,
    configuration: &RenderConfiguration,
) -> Result<Vec<Value>, StatusCode>
where
    PathT: AsRef<Path>,
{
    let mut catalog = Vec::new();

    let mut files = read_dir(directory).await.map_err_internal_server("read directory")?;
    while let Some(file) = files.next_entry().await.map_err_internal_server("directory entry")? {
        let file_path = file.path();
        let (file_name, file_name_without_extension) = file_name(&file_path);
        if file_name_without_extension != INDEX {
            if let Some(rendered_page_type) = configuration.is_rendered_page(&file_name) {
                let rendered_page = RenderedPage::new_from_file(rendered_page_type, &file_path, configuration)
                    .await
                    .map_err_internal_server("read render file")?;

                let title = rendered_page.title(configuration)?;
                let title: &str = title.as_ref().map(|title| title.as_ref()).unwrap_or(&file_name_without_extension);

                let href = uri_path_join(uri_path, &file_name_without_extension);

                let created = rendered_page
                    .annotations
                    .created
                    .as_ref()
                    .map(|date_time| date_time.value.timestamp())
                    .unwrap_or_default();

                let updated = rendered_page
                    .annotations
                    .updated
                    .as_ref()
                    .map(|date_time| date_time.value.timestamp())
                    .unwrap_or_default();

                let mut item = Map::new();

                item.value.insert("title".into(), title.into());
                item.value.insert("href".into(), href.into());
                item.value.insert("created".into(), created.into());
                item.value.insert("updated".into(), updated.into());

                for (key, column) in &annotation.extra_columns {
                    if let Some(value) = rendered_page.annotations.traverse_variable(column) {
                        item.value.insert((*key).into(), value.clone());
                    }
                }

                catalog.push(item.into());
            }
        }
    }

    annotation.sort(&mut catalog);

    Ok(catalog)
}
