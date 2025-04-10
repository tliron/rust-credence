use super::{
    super::{super::util::*, add_on::*, context::*},
    annotation::*,
    create::*,
};

use axum::{http::StatusCode, response::*};

//
// Catalog
//

/// Catalog add on.
pub struct Catalog;

use async_trait::*;

#[async_trait]
impl RenderHandler for Catalog {
    async fn render<'own>(&self, context: &mut RenderContext<'own>) -> Result<(), StatusCode> {
        if let Some(catalog) = context.rendered_page.annotations.other.get("catalog") {
            let catalog_annotation = CatalogAnnotation::resolve(catalog);
            let uri_path = uri_path_parent(context.uri_path);
            let directory = context.configuration.paths.asset(&uri_path);
            let catalog =
                create_catalog(catalog_annotation, &uri_path, directory, &context.configuration.render).await?;
            context.values.insert("catalog".into(), catalog.into());
        }

        Ok(())
    }
}
