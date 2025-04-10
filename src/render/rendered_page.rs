use super::{super::configuration::*, annotations::*, constants::*, templates::*};

use {
    axum::{
        http::{HeaderMap, StatusCode},
        response::*,
    },
    chrono::*,
    compris::{normal::*, *},
    kutil_http::*,
    kutil_std::string::ParseError,
    std::{io, path::*, str::*, time::*},
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
    pub content: Option<String>,
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

        let (annotations, content) = Self::parse(rendered_page_type, &body, configuration);

        Ok(Self {
            headers,
            annotations,
            content,
        })
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

        let (annotations, content) = Self::parse(rendered_page_type, &string, configuration);

        Ok(Self {
            headers: HeaderMap::new(),
            annotations,
            content,
        })
    }

    /// Parse.
    pub fn parse(
        rendered_page_type: RenderedPageType,
        string: &str,
        configuration: &RenderConfiguration,
    ) -> (Annotations, Option<String>) {
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

    /// Catalog.
    pub async fn catalog<PathT>(
        uri_path: &str,
        directory: PathT,
        sort: Sort,
        configuration: &RenderConfiguration,
    ) -> Result<Vec<Value>, StatusCode>
    where
        PathT: AsRef<Path>,
    {
        let mut catalog = Vec::new();

        for file_path in directory
            .as_ref()
            .read_dir()
            .map_err_internal_server("catalog")?
        {
            let file_path = file_path.map_err_internal_server("path")?;
            let file_path = file_path.path();
            let (filename, filename_without_extension) = filename(&file_path);
            if filename_without_extension != INDEX {
                if let Some(rendered_page_type) = configuration.is_rendered_page(&filename) {
                    let rendered_page =
                        Self::new_from_file(rendered_page_type, &file_path, configuration)
                            .await
                            .map_err_internal_server("read render file")?;

                    let title = rendered_page
                        .title(configuration)
                        .await?
                        .unwrap_or_else(|| filename_without_extension.clone());

                    let href = String::from(uri_path) + &filename_without_extension;

                    let created = rendered_page
                        .annotations
                        .created
                        .map(|d| d.value.timestamp())
                        .unwrap_or_default();

                    let updated = rendered_page
                        .annotations
                        .updated
                        .map(|d| d.value.timestamp())
                        .unwrap_or_default();

                    let item = normal_map!(
                        ("title", title),
                        ("href", href),
                        ("created", created),
                        ("updated", updated)
                    );

                    catalog.push(item.into());
                }
            }
        }

        match sort {
            Sort::Alpha => {
                let title = "title".into();
                catalog.sort_by(|a: &Value, b: &Value| {
                    if let Value::Text(a) = a.get(&title).expect("title") {
                        if let Value::Text(b) = b.get(&title).expect("title") {
                            return a.value.to_lowercase().cmp(&b.value.to_lowercase());
                        }
                    }
                    panic!("impossible");
                })
            }

            Sort::Created => {
                let created = "created".into();
                catalog.sort_by(|a: &Value, b: &Value| {
                    if let Value::Integer(a) = a.get(&created).expect("created") {
                        if let Value::Integer(b) = b.get(&created).expect("created") {
                            return a.value.cmp(&b.value);
                        }
                    }
                    panic!("impossible");
                })
            }

            Sort::Updated => {
                let updated = "updated".into();
                catalog.sort_by(|a: &Value, b: &Value| {
                    if let Value::Integer(a) = a.get(&updated).expect("updated") {
                        if let Value::Integer(b) = b.get(&updated).expect("updated") {
                            return a.value.cmp(&b.value);
                        }
                    }
                    panic!("impossible");
                })
            }
        }

        Ok(catalog)
    }

    /// To response.
    pub async fn to_response(
        mut self,
        path: &str,
        templates: &Templates,
        configuration: &ServerConfiguration,
    ) -> Result<Response, StatusCode> {
        if !self.annotations.values.contains_key("content") {
            let content = self
                .render_content(&configuration.render)
                .await?
                .unwrap_or_default();

            self.annotations
                .values
                .insert("content".into(), content.into());
        }

        if !self.annotations.values.contains_key("title") {
            let mut set = false;
            if let Some(content) = self.annotations.values.get("content") {
                if let Value::Text(content) = content {
                    self.annotations.values.insert(
                        "title".into(),
                        title_from_content(content.into()).unwrap_or("").into(),
                    );
                    set = true;
                }
            }

            if !set {
                self.annotations.values.insert("title".into(), "".into());
            }
        }

        if !self.annotations.values.contains_key("created") {
            if let Some(created) = &self.annotations.created {
                self.annotations.values.insert(
                    "created".into(),
                    created.value.format(DATE_TIME_FORMAT).to_string().into(),
                );
            }
        }

        if !self.annotations.values.contains_key("updated") {
            if let Some(updated) = self.updated() {
                self.annotations.values.insert(
                    "updated".into(),
                    updated.format(DATE_TIME_FORMAT).to_string().into(),
                );
            }
        }

        if let Some(catalog) = self.annotations.other.get("catalog") {
            let sort = match traverse!(catalog, "sort") {
                Some(sort) => sort.to_string().parse().unwrap_or_default(),
                None => Sort::default(),
            };

            let last_slash = path.rfind(PATH_SEPARATOR).unwrap_or(1);
            let uri_path = String::from(&path[..last_slash]) + PATH_SEPARATOR_STRING;
            let directory = configuration.paths.asset(&uri_path);
            let catalog = Self::catalog(&uri_path, directory, sort, &configuration.render).await?;
            self.annotations
                .values
                .insert("catalog".into(), catalog.into());
        }

        let html = self.render_template(templates, configuration).await?;
        let headers = self.merge_headers()?;

        response_from_bytes(html.into(), "text/html", headers)
    }

    /// Render content.
    pub async fn render_content(
        &self,
        configuration: &RenderConfiguration,
    ) -> Result<Option<String>, StatusCode> {
        match self.content.as_ref() {
            Some(content) => self
                .annotations
                .renderer(configuration)
                .render(content)
                .await
                .map(Some),

            None => Ok(None),
        }
    }

    /// Render template.
    pub async fn render_template(
        &self,
        templates: &Templates,
        configuration: &ServerConfiguration,
    ) -> Result<Vec<u8>, StatusCode> {
        templates
            .render(
                self.annotations.template(&configuration.render),
                &self.annotations.values,
            )
            .await
    }

    /// Merge annotations headers into headers.
    pub fn merge_headers(self) -> Result<HeaderMap, StatusCode> {
        let mut headers = self.headers;
        headers
            .set_string_values(self.annotations.headers.into_iter())
            .map_err_internal_server("header value")?;
        Ok(headers)
    }

    /// From annotations or fallback to finding the title in the content.
    pub async fn title(
        &self,
        configuration: &RenderConfiguration,
    ) -> Result<Option<String>, StatusCode> {
        Ok(match self.annotations.values.get("title") {
            Some(title) => Some(title.to_string()),
            None => match self.render_content(configuration).await? {
                Some(content) => title_from_content(&content).map(|title| title.into()),
                None => None,
            },
        })
    }

    /// From annotations or fallback to `Last-Modified` header.
    pub fn updated(&self) -> Option<DateTime<Utc>> {
        self.annotations
            .updated
            .as_ref()
            .map(|last_modified| last_modified.value.into())
            .or_else(|| {
                self.headers.last_modified().map(|last_modified| {
                    let last_modified: SystemTime = last_modified.into();
                    last_modified.into()
                })
            })
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

/// Filename with extension and without extension.
pub fn filename<PathT>(path: PathT) -> (String, String)
where
    PathT: AsRef<Path>,
{
    let path = path.as_ref().to_string_lossy();

    let mut filename = path.as_ref();
    if let Some(last_slash) = filename.rfind(MAIN_SEPARATOR) {
        filename = &filename[last_slash + 1..];
    }

    let mut filename_without_extension = filename;
    if let Some(first_dot) = filename_without_extension.find('.') {
        filename_without_extension = &filename_without_extension[..first_dot];
    }

    (filename.into(), filename_without_extension.into())
}

//
// Sort
//

/// Sort.
#[derive(Debug, Default, Clone, Copy)]
pub enum Sort {
    #[default]
    Alpha,
    Created,
    Updated,
}

impl FromStr for Sort {
    type Err = ParseError;

    fn from_str(representation: &str) -> Result<Self, Self::Err> {
        match representation.to_lowercase().as_str() {
            "alpha" => Ok(Self::Alpha),
            "created" => Ok(Self::Created),
            "updated" => Ok(Self::Updated),
            _ => Err(format!("unsupported: {}", representation).into()),
        }
    }
}
