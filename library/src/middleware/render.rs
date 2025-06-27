use super::{
    super::{configuration::*, render::*},
    defer::*,
    socket::*,
};

use {
    ::axum::{
        extract::{Request, *},
        http::{header::*, *},
        middleware::*,
        response::Response,
    },
    bytestring::*,
    kutil_http::*,
    std::{cmp::*, result::Result},
};

//
// RenderMiddleware
//

/// Axum middleware that renders pages.
#[derive(Clone, Debug)]
pub struct RenderMiddleware {
    /// Configuration.
    pub configuration: CredenceConfiguration,

    /// Templates.
    pub templates: Templates,
}

impl RenderMiddleware {
    /// Constrctor.
    pub fn new(configuration: CredenceConfiguration) -> Self {
        let templates = configuration.files.templates();
        Self::new_with(configuration, templates)
    }

    /// Constructor.
    pub fn new_with(configuration: CredenceConfiguration, templates: Templates) -> Self {
        Self { configuration, templates }
    }

    /// To be used with [from_fn_with_state].
    pub async fn function(
        State(state_self): State<Self>,
        mut request: Request,
        next: Next,
    ) -> Result<Response, StatusCode> {
        let uri_path: ByteString = match request.uri().decoded_path() {
            Some(uri_path) => uri_path.as_ref().into(),
            None => {
                // Cannot decode path
                tracing::error!("cannot decode path: {}", request.uri());
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };

        if let Some(rendered_page_type) = state_self.configuration.render.is_rendered_page(&uri_path) {
            // Negotiate
            let mut last_modified = None;
            if let Some(if_modified_since) = request.headers().if_modified_since() {
                let path = state_self.configuration.files.asset(&uri_path);
                let mut modified = file_modified(&path).map_err_internal_server("file modified")?;

                if let Some(coordinator_modified) = state_self
                    .configuration
                    .files
                    .coordinate
                    .coordinator_modified()
                    .map_err_internal_server("coordinator modified")?
                {
                    modified = max(modified, coordinator_modified);
                }

                if modified_since(Some(modified), Some(if_modified_since)) {
                    last_modified = Some(modified);

                    // Don't let next service do conditional HTTP
                    let headers = request.headers_mut();
                    headers.remove(IF_MODIFIED_SINCE);
                    headers.remove(IF_NONE_MATCH);
                }
            }

            let original_uri_path = DeferredResponse::get(&request).and_then(|deferred_response| {
                if let DeferredResponse::RewriteFrom(original_uri_path) = deferred_response {
                    Some(original_uri_path.clone())
                } else {
                    None
                }
            });

            let is_json = is_json(&request);

            let query = request.uri().decoded_query_map();

            let socket = request.extensions().get::<Socket>().cloned();

            let response = next.run(request).await;

            if response.status() == StatusCode::OK {
                tracing::debug!("rendered page: {}", uri_path);

                let rendered_page = RenderedPage::new_from_response(
                    &uri_path,
                    rendered_page_type,
                    response,
                    &state_self.configuration.render,
                )
                .await?;

                let mut context = rendered_page.context(
                    socket,
                    uri_path,
                    original_uri_path,
                    query,
                    last_modified,
                    is_json,
                    &state_self.templates,
                    &state_self.configuration,
                );

                context.prepare_default().await?;
                context.prepare(CatalogPreparer).await?;

                context.into_response().await
            } else {
                Ok(response)
            }
        } else {
            Ok(next.run(request).await)
        }
    }
}

fn is_json(request: &Request) -> (bool, bool) {
    if request
        .headers()
        .accept()
        .best(RENDERED_PAGE_MEDIA_TYPES)
        .map(|media_type_selector| media_type_selector == &JSON_MEDIA_TYPE)
        .unwrap_or_default()
    {
        if let Some(query) = request.uri().decoded_query_map() {
            return (true, query.get_single_as_ref("pretty") == Some("true"));
        }

        (true, false)
    } else {
        (false, false)
    }
}
