use super::{
    super::{configuration::*, middleware::*},
    annotations::*,
    context::*,
    templates::*,
};

use {
    ::axum::{http::*, response::Response},
    compris::{normal::*, *},
    httpdate::*,
    kutil_http::*,
    kutil_std::{error::*, zerocopy::*},
    std::{io, path::*, result::Result},
    tokio::{fs::*, io::*},
};

//
// RenderedPageType
//

/// Rendered page type.
#[derive(Clone, Copy, Debug)]
pub enum RenderedPageType {
    /// Content with optional embedded annotations.
    ContentWithEmbeddedAnnotations,

    /// Just annotations.
    Annotations(Format),
}

//
// RenderedPage
//

/// Rendered page.
#[derive(Clone, Debug)]
pub struct RenderedPage {
    /// Headers.
    pub headers: HeaderMap,

    /// Annotations.
    pub annotations: Annotations,

    /// Content.
    pub content: Option<ByteString>,
}

impl RenderedPage {
    /// Constructor.
    pub async fn new_from_response(
        identifier: &str,
        rendered_page_type: RenderedPageType,
        response: Response,
        configuration: &RenderConfiguration,
    ) -> Result<Self, StatusCode> {
        let headers = response.headers().clone();
        let body = response.into_body();

        let (body, _trailers) = body
            .read_into_string(configuration.max_content_size.inner.into())
            .await
            .map_err_internal_server("read body into string")?;

        let (annotations, content) = Self::split(identifier, rendered_page_type, &body, configuration);

        Ok(Self { headers, annotations, content })
    }

    /// Constructor.
    pub async fn new_from_file<PathT>(
        rendered_page_type: RenderedPageType,
        path: PathT,
        configuration: &RenderConfiguration,
    ) -> io::Result<Self>
    where
        PathT: AsRef<Path>,
    {
        let path = path.as_ref();
        let mut file = File::open(path).await.with_path(path)?;
        let mut string = String::default();
        file.read_to_string(&mut string).await?;

        let (annotations, content) =
            Self::split(path.to_string_lossy().as_ref(), rendered_page_type, &string, configuration);

        Ok(Self { headers: Default::default(), annotations, content })
    }

    /// Create a [RenderContext].
    pub fn context<'own>(
        &'own self,
        socket: Option<Socket>,
        uri_path: ByteString,
        original_uri_path: Option<ByteString>,
        query: Option<QueryMap>,
        last_modified: Option<HttpDate>,
        is_json: (bool, bool),
        templates: &'own Templates,
        configuration: &'own CredenceConfiguration,
    ) -> RenderContext<'own> {
        // Our variables override global variables
        let mut variables = configuration.render.variables.clone();
        for (key, value) in &self.annotations.variables {
            variables.insert(key.clone(), value.clone());
        }

        RenderContext::new(
            self,
            variables,
            socket,
            uri_path,
            original_uri_path,
            query,
            last_modified,
            is_json,
            self.annotations.renderer(&configuration.render).clone(),
            templates,
            configuration,
        )
    }

    /// Merge annotations headers into headers.
    pub fn merged_headers(&self) -> Result<HeaderMap, StatusCode> {
        let mut headers = self.headers.clone();
        headers.set_string_values(self.annotations.headers.iter()).map_err_internal_server("header value")?;
        Ok(headers)
    }

    /// Get the title from the annotations or fallback to extracting it from the content.
    pub fn title(&self, configuration: &RenderConfiguration) -> Result<Option<ByteString>, StatusCode> {
        Ok(match self.annotations.variables.get("title") {
            Some(title) => match title {
                Variant::Text(title) => Some(title.inner.clone()),
                _ => None,
            },

            None => {
                let renderer = self.annotations.renderer(configuration);
                match self.content.as_ref() {
                    Some(content) => renderer.title_from_content(&content)?,
                    None => None,
                }
            }
        })
    }

    /// Split [Annotations] from content.
    pub fn split(
        identifier: &str,
        rendered_page_type: RenderedPageType,
        string: &str,
        configuration: &RenderConfiguration,
    ) -> (Annotations, Option<ByteString>) {
        match rendered_page_type {
            RenderedPageType::Annotations(format) => {
                let annotations = Annotations::parse(identifier, &string, format);
                (annotations, None)
            }

            RenderedPageType::ContentWithEmbeddedAnnotations => {
                let (annotations, content) = configuration.annotations.split(identifier, &string);
                (annotations, Some(content.into()))
            }
        }
    }
}
