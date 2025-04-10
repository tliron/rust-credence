use super::super::{configuration::*, render::*};

use axum::{extract::*, http::StatusCode, middleware::*, response::*};

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
        Self {
            configuration,
            templates,
        }
    }

    /// To be used with [from_fn_with_state].
    pub async fn function(
        State(state_self): State<Self>,
        request: Request,
        next: Next,
    ) -> Result<Response, StatusCode> {
        let path = request.uri().path().to_owned();

        let response = next.run(request).await;

        if let Some(rendered_page_type) = state_self.configuration.render.is_rendered_page(&path) {
            if response.status() == StatusCode::OK {
                tracing::debug!("rendered page: {}", path);

                let rendered_page = RenderedPage::new_from_response(
                    rendered_page_type,
                    response,
                    &state_self.configuration.render,
                )
                .await?;

                return rendered_page
                    .to_response(&path, &state_self.templates, &state_self.configuration)
                    .await;
            }
        }

        Ok(response)
    }
}
