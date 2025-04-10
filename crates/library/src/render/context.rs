use super::{super::configuration::*, constants::*, rendered_page::*, templates::*};

use {
    axum::{
        http::{StatusCode, header::*},
        response::*,
    },
    bytestring::*,
    compris::{normal::*, ser::*, *},
    httpdate::*,
    kutil_http::*,
    kutil_std::collections::*,
};

/// Render context.
pub struct RenderContext<'own> {
    /// Rendered page.
    pub rendered_page: &'own RenderedPage,

    /// URI path.
    pub uri_path: &'own str,

    /// Last modified.
    pub last_modified: Option<HttpDate>,

    /// JSON?
    pub json: bool,

    /// Templates.
    pub templates: &'own Templates,

    /// Configuration.
    pub configuration: &'own ServerConfiguration,

    /// Values.
    pub values: FastHashMap<ByteString, Value>,
}

impl<'own> RenderContext<'own> {
    /// Constructor.
    pub fn new(
        rendered_page: &'own RenderedPage,
        uri_path: &'own str,
        last_modified: Option<HttpDate>,
        json: bool,
        templates: &'own Templates,
        configuration: &'own ServerConfiguration,
        values: FastHashMap<ByteString, Value>,
    ) -> Self {
        Self { rendered_page, uri_path, last_modified, json, templates, configuration, values }
    }

    /// To response.
    pub async fn to_response(self) -> Result<Response, StatusCode> {
        let template = self.rendered_page.annotations.template(&self.configuration.render);
        let html = self.templates.render(template, &self.values).await?;
        let mut headers = self.rendered_page.merged_headers()?;

        if let Some(last_modified) = &self.last_modified {
            headers
                .set_string_value(LAST_MODIFIED, &last_modified.to_string())
                .map_err_internal_server("set Last-Modified")?;
        }

        if self.json {
            let values: Value = self.values.into_iter().map(|(key, value)| (key.into(), value.clone())).collect();
            let json = Serializer::new(Format::JSON)
                .with_pretty(true)
                .stringify_modal(&values, &SerializationMode::for_json())
                .map_err_internal_server("serialize JSON")?;
            return response_from_bytes(json.into(), JSON_MEDIA_TYPE_STRING, headers);
        }

        response_from_bytes(html.into_bytes(), HTML_MEDIA_TYPE_STRING, headers)
    }
}
