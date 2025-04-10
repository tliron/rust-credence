use super::{
    super::{configuration::*, middleware::*},
    constants::*,
    default_preparer::*,
    preparer::*,
    rendered_page::*,
    renderer::*,
    templates::*,
};

use {
    ::axum::{
        http::{header::*, *},
        response::Response,
    },
    bytestring::*,
    compris::{normal::*, ser::*, *},
    httpdate::*,
    kutil_http::*,
    kutil_std::collections::*,
    std::result::Result,
};

/// Render context.
#[derive(Debug)]
pub struct RenderContext<'own> {
    /// Rendered page.
    pub rendered_page: &'own RenderedPage,

    /// Variables.
    pub variables: FastHashMap<ByteString, Value>,

    /// Socket.
    pub socket: Option<Socket>,

    /// URI path.
    pub uri_path: ByteString,

    /// Original URI path.
    pub original_uri_path: Option<ByteString>,

    /// Last modified.
    pub last_modified: Option<HttpDate>,

    /// Is JSON? And if so, is it pretty JSON?
    pub is_json: (bool, bool),

    /// Renderer
    pub renderer: Renderer,

    /// Templates.
    pub templates: &'own Templates,

    /// Configuration.
    pub configuration: &'own CredenceConfiguration,
}

impl<'own> RenderContext<'own> {
    /// Constructor.
    pub fn new(
        rendered_page: &'own RenderedPage,
        variables: FastHashMap<ByteString, Value>,
        socket: Option<Socket>,
        uri_path: ByteString,
        original_uri_path: Option<ByteString>,
        last_modified: Option<HttpDate>,
        is_json: (bool, bool),
        renderer: Renderer,
        templates: &'own Templates,
        configuration: &'own CredenceConfiguration,
    ) -> Self {
        Self {
            rendered_page,
            variables,
            socket,
            uri_path,
            original_uri_path,
            last_modified,
            is_json,
            renderer,
            templates,
            configuration,
        }
    }

    /// Prepare using [DefaultRenderedPageHandler].
    pub async fn prepare<PreparerT>(&mut self, preparer: PreparerT) -> Result<(), StatusCode>
    where
        PreparerT: RenderPreparer,
    {
        preparer.prepare(self).await
    }

    /// Prepare using [DefaultRenderedPageHandler].
    pub async fn prepare_default(&mut self) -> Result<(), StatusCode> {
        self.prepare(DefaultRenderedPageHandler).await
    }

    /// Into response.
    pub async fn into_response(self) -> Result<Response, StatusCode> {
        let template = self.rendered_page.annotations.template(&self.configuration.render);
        let html = self.templates.render(template, &self.variables).await?;
        let mut headers = self.rendered_page.merged_headers()?;

        if let Some(last_modified) = &self.last_modified {
            headers
                .set_string_value(LAST_MODIFIED, &last_modified.to_string())
                .map_err_internal_server("set Last-Modified")?;
        }

        if self.is_json.0 {
            let json = self.into_json()?;
            response_from_bytes(json.into_bytes(), JSON_MEDIA_TYPE_STRING, headers)
        } else {
            response_from_bytes(html.into_bytes(), HTML_MEDIA_TYPE_STRING, headers)
        }
    }

    fn into_json(self) -> Result<ByteString, StatusCode> {
        Serializer::new(Format::JSON)
            .with_pretty(self.is_json.1)
            .stringify_modal(&self.variables_into_value(), &SerializationMode::for_json())
            .map_err_internal_server("serialize JSON")
    }

    fn variables_into_value(self) -> Value {
        self.variables.into_iter().map(|(key, value)| (key.into(), value.clone())).collect()
    }
}
