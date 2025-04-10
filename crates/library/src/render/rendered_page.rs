use super::{
    super::{configuration::*, util::*},
    add_on::*,
    annotations::*,
    catalog::*,
    context::*,
    templates::*,
};

use {
    axum::{
        http::{HeaderMap, StatusCode},
        response::*,
    },
    bytestring::*,
    compris::{normal::*, *},
    httpdate::*,
    kutil_http::*,
    kutil_std::collections::*,
    std::{io, path::*},
    tokio::{fs::*, io::AsyncReadExt},
};

const DATE_TIME_FORMAT: &str = "%B %-d, %Y";

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
        rendered_page_type: RenderedPageType,
        response: Response,
        configuration: &RenderConfiguration,
    ) -> Result<Self, StatusCode> {
        let headers = response.headers().clone();
        let body = response.into_body();

        let (body, _trailers) = body
            .read_into_string(configuration.max_content_size.value.into())
            .await
            .map_err_internal_server("read body into string")?;

        let (annotations, content) = Self::split(rendered_page_type, &body, configuration);

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
        let mut file = File::open(path).await?;
        let mut string = String::new();
        file.read_to_string(&mut string).await?;

        let (annotations, content) = Self::split(rendered_page_type, &string, configuration);

        Ok(Self { headers: HeaderMap::new(), annotations, content })
    }

    /// To response.
    pub async fn to_response(
        &self,
        uri_path: &str,
        original_uri_path: Option<ByteString>,
        last_modified: Option<HttpDate>,
        json: bool,
        templates: &Templates,
        configuration: &ServerConfiguration,
    ) -> Result<Response, StatusCode> {
        let mut context = RenderContext::new(
            self,
            uri_path,
            last_modified,
            json,
            templates,
            configuration,
            self.annotations.values.clone(),
        );

        if !context.values.contains_key("content") {
            let content = self.render_content(&configuration.render).await?.unwrap_or_default();
            context.values.insert("content".into(), content.into());
        }

        if !context.values.contains_key("title") {
            if let Some(content) = context.values.get("content") {
                if let Value::Text(content) = content {
                    let title = title_from_content(content.as_str()).unwrap_or("");
                    context.values.insert("title".into(), title.into());
                }
            }
        }

        if !context.values.contains_key("created") {
            if let Some(created) = &self.annotations.created {
                let created = created.value.format(DATE_TIME_FORMAT).to_string();
                context.values.insert("created".into(), created.into());
            }
        }

        if !context.values.contains_key("updated") {
            if let Some(updated) = &self.annotations.updated {
                let updated = updated.value.format(DATE_TIME_FORMAT).to_string();
                context.values.insert("updated".into(), updated.into());
            }
        }

        if !context.values.contains_key("up") {
            if let Some(original_path) = original_uri_path {
                let up = uri_path_parent(&original_path);
                if up != PATH_SEPARATOR_STRING {
                    context.values.insert("up".into(), up.into());
                }
            }
        }

        // fn up() -> impl AsyncFn(&mut RenderContext<'_>) -> Result<(), StatusCode> {
        //     async |context| -> Result<(), StatusCode> {
        //         if !context.values.contains_key("up") {
        //             if let Some(original_path) = original_uri_path {
        //                 let up = uri_path_parent(&original_path);
        //                 if up != PATH_SEPARATOR_STRING {
        //                     context.values.insert("up".into(), up.into());
        //                 }
        //             }
        //         }
        //     }
        // }

        // fn catalog() -> impl AsyncFn(&mut RenderContext<'_>) -> Result<(), StatusCode> {
        //     Catalog::render
        // }

        // let add_ons = vec![up(), catalog()];

        // let mut add_ons: Vec<Box<dyn RenderAddOn>> = Vec::new();
        // add_ons.push(Box::new(Catalog));

        // for add_on in add_ons {
        //     add_on.render(&mut context).await?;
        // }

        Catalog.render(&mut context).await?;

        context.to_response().await
    }

    /// Render content.
    pub async fn render_content(&self, configuration: &RenderConfiguration) -> Result<Option<ByteString>, StatusCode> {
        match self.content.as_ref() {
            Some(content) => self.annotations.renderer(configuration).render(content).await.map(Some),
            None => Ok(None),
        }
    }

    /// Clone the values map using references to the items.
    pub fn clone_values_as_ref(&self) -> FastHashMap<&str, &Value> {
        self.annotations
            .values
            .iter()
            .map(|(key, value)| {
                let key: &str = key;
                (key, value)
            })
            .collect()
    }

    /// Merge annotations headers into headers.
    pub fn merged_headers(&self) -> Result<HeaderMap, StatusCode> {
        let mut headers = self.headers.clone();
        headers.set_string_values(self.annotations.headers.iter()).map_err_internal_server("header value")?;
        Ok(headers)
    }

    /// From annotations or fallback to finding the title in the content.
    pub async fn title(&self, configuration: &RenderConfiguration) -> Result<Option<String>, StatusCode> {
        Ok(match self.annotations.values.get("title") {
            Some(title) => Some(title.to_string()),
            None => match self.render_content(configuration).await? {
                Some(content) => title_from_content(&content).map(|title| title.into()),
                None => None,
            },
        })
    }

    /// Split [Annotations] from content.
    pub fn split(
        rendered_page_type: RenderedPageType,
        string: &str,
        configuration: &RenderConfiguration,
    ) -> (Annotations, Option<ByteString>) {
        match rendered_page_type {
            RenderedPageType::Annotations(format) => {
                let annotations = Annotations::parse(&string, format);
                (annotations, None)
            }

            RenderedPageType::ContentWithEmbeddedAnnotations => {
                let (annotations, content) = configuration.annotations.split(&string);
                (annotations, Some(content.into()))
            }
        }
    }
}

/// First `<h1>`.
pub fn title_from_content(content: &str) -> Option<&str> {
    if let Some(heading_start) = content.find("<h1>") {
        let title = &content[heading_start + 4..];
        if let Some(heading_end) = title.find("</h1>") {
            let title = &title[..heading_end];
            return Some(title);
        }
    }

    None
}
