use super::{super::util::*, context::*, preparer::*};

use {axum::http::*, compris::normal::*, std::result::Result};

/// Default [RenderPreparer].
///
/// If the following values are not already set, will set them:
///
/// * `content`: will render it
/// * `title`: will extract it from the `content` (the first heading)
/// * `created`: will use the annotation if set
/// * `updated`: will use the annotation if set
/// * `up`: will use the original URI path's parent if it is not "/"
pub struct DefaultRenderedPageHandler;

impl RenderPreparer for DefaultRenderedPageHandler {
    async fn prepare<'own>(&self, context: &mut RenderContext<'own>) -> Result<(), StatusCode> {
        context.renderer.clone().prepare(context).await?;

        if !context.values.contains_key("title") {
            let title = match context.values.get("content") {
                Some(content) => match content {
                    Value::Text(content) => context.renderer.title_from_content(&content.value)?,
                    _ => None,
                },
                None => None,
            }
            .unwrap_or("".into());

            context.values.insert("title".into(), title.into());
        }

        if !context.values.contains_key("created") {
            if let Some(created) = &context.rendered_page.annotations.created {
                let created = created.value.timestamp();
                context.values.insert("created".into(), created.into());
            }
        }

        if !context.values.contains_key("updated") {
            if let Some(updated) = &context.rendered_page.annotations.updated {
                let updated = updated.value.timestamp();
                context.values.insert("updated".into(), updated.into());
            }
        }

        if !context.values.contains_key("up") {
            if let Some(original_path) = &context.original_uri_path {
                if original_path != PATH_SEPARATOR_STRING {
                    let up = uri_path_parent(original_path);
                    context.values.insert("up".into(), up.into());
                }
            }
        }

        Ok(())
    }
}
