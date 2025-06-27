use super::{
    super::{super::util::*, context::*, preparer::*},
    annotation::*,
    create::*,
};

use {axum::http::*, std::result::Result};

//
// Catalog
//

/// [RenderPreparer] for catalog.
pub struct CatalogPreparer;

impl RenderPreparer for CatalogPreparer {
    async fn prepare<'own>(&self, context: &mut RenderContext<'own>) -> Result<(), StatusCode> {
        if let Some(catalog) = context.rendered_page.annotations.other.get("catalog") {
            let annotation = CatalogAnnotation::resolve(catalog);
            let uri_path = uri_path_parent(&context.uri_path);
            let directory = context.configuration.files.asset(&uri_path);
            let catalog = create_catalog(annotation, &uri_path, directory, &context.configuration.render).await?;
            context.variables.insert("catalog".into(), catalog.into());
        }

        Ok(())
    }
}
