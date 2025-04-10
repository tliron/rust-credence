use super::{
    super::{configuration::*, render::*},
    defer::*,
};

use {
    axum::{
        extract::*,
        http::{StatusCode, header::*},
        middleware::*,
        response::*,
    },
    kutil_http::*,
    std::cmp::*,
};

//
// RenderMiddleware
//

/// Axum middleware that renders pages.
#[derive(Clone, Debug)]
pub struct RenderMiddleware {
    /// Configuration.
    pub configuration: ServerConfiguration,

    /// Templates.
    pub templates: Templates,
}

impl RenderMiddleware {
    /// Constrctor.
    pub fn new(configuration: ServerConfiguration) -> Self {
        let templates = configuration.paths.templates();
        Self::new_with(configuration, templates)
    }

    /// Constructor.
    pub fn new_with(configuration: ServerConfiguration, templates: Templates) -> Self {
        Self { configuration, templates }
    }

    /// To be used with [from_fn_with_state].
    pub async fn function(
        State(state_self): State<Self>,
        mut request: Request,
        next: Next,
    ) -> Result<Response, StatusCode> {
        //return Err(StatusCode::INTERNAL_SERVER_ERROR);
        let uri_path = request.uri().decoded_path().map_err_internal_server("decode path")?.into_owned();

        let original_uri_path = DeferredResponse::get(&request).and_then(|deferred_response| {
            if let DeferredResponse::RewriteFrom(original_uri_path) = deferred_response {
                Some(original_uri_path.clone())
            } else {
                None
            }
        });

        if let Some(rendered_page_type) = state_self.configuration.render.is_rendered_page(&uri_path) {
            // Negotiate
            let mut last_modified = None;
            if let Some(if_modified_since) = request.headers().if_modified_since() {
                let path = state_self.configuration.paths.asset(&uri_path);
                let mut modified = file_modified(&path).map_err_internal_server("file modified")?;

                if let Some(coordinator_modified) = state_self
                    .configuration
                    .paths
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

            let json = is_json(request.headers());

            let response = next.run(request).await;

            if response.status() == StatusCode::OK {
                tracing::debug!("rendered page: {}", uri_path);

                let rendered_page =
                    RenderedPage::new_from_response(rendered_page_type, response, &state_self.configuration.render)
                        .await?;

                rendered_page
                    .to_response(
                        &uri_path,
                        original_uri_path,
                        last_modified,
                        json,
                        &state_self.templates,
                        &state_self.configuration,
                    )
                    .await
            } else {
                Ok(response)
            }
        } else {
            Ok(next.run(request).await)
        }
    }
}

fn is_json(request_headers: &HeaderMap) -> bool {
    request_headers
        .accept()
        .best(RENDERED_PAGE_MEDIA_TYPES)
        .map(|media_type_selector| media_type_selector == &JSON_MEDIA_TYPE)
        .unwrap_or_default()
}
