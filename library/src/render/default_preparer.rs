use super::{super::util::*, context::*, preparer::*};

use {axum::http::*, std::result::Result};

/// Default [RenderPreparer].
///
/// If the following variables are not already set, will set them:
///
/// * `socket`: the [Socket](super::super::middleware::Socket)
/// * `content`: will render it
/// * `title`: will extract it from the `content` (the first heading)
/// * `created`: will use the annotation if set
/// * `updated`: will use the annotation if set
/// * `path`: the original URI path
/// * `query`: the URI query
pub struct DefaultRenderedPageHandler;

impl RenderPreparer for DefaultRenderedPageHandler {
    async fn prepare<'own>(&self, context: &mut RenderContext<'own>) -> Result<(), StatusCode> {
        if let Some(socket) = &context.socket {
            context.variables.insert("socket".into(), socket.into());
        }

        context.renderer.clone().prepare(context).await?;

        if !context.variables.contains_key("title") {
            let title = match &context.rendered_page.content {
                Some(content) => context.renderer.title_from_content(content)?,
                None => None,
            }
            .unwrap_or("".into());

            context.variables.insert("title".into(), title.into());
        }

        if !context.variables.contains_key("created") {
            if let Some(created) = &context.rendered_page.annotations.created {
                let created = created.inner.timestamp();
                context.variables.insert("created".into(), created.into());
            }
        }

        if !context.variables.contains_key("updated") {
            if let Some(updated) = &context.rendered_page.annotations.updated {
                let updated = updated.inner.timestamp();
                context.variables.insert("updated".into(), updated.into());
            }
        }

        if !context.variables.contains_key("path") {
            let original_path = context.original_uri_path.clone().unwrap_or_default();
            context.variables.insert("path".into(), original_path.into());
        }

        if !context.variables.contains_key("query") {
            if let Some(query) = &context.query {
                context.variables.insert("query".into(), query_map_to_variant(query));
            }
        }

        Ok(())
    }
}
