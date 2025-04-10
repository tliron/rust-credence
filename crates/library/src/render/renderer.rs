use {axum::http::StatusCode, bytestring::*, compris::*, kutil_http::*, kutil_std::*, markdown::*};

//
// Renderer
//

/// Renderer.
#[derive(Clone, Copy, Debug, Default, FromStr)]
#[from_str(lowercase)]
pub enum Renderer {
    /// Passthrough.
    Passthrough,

    /// Markdown.
    #[strings("markdown", "md")]
    Markdown,

    /// GitHub-Flavored Markdown.
    #[default]
    GFM,
}

impl Renderer {
    /// Render.
    pub async fn render(&self, content: &str) -> Result<ByteString, StatusCode> {
        match self {
            Self::Passthrough => Ok(content.into()),
            Self::Markdown => Self::render_markdown(content, Options::default()),
            Self::GFM => Self::render_markdown(content, Options::gfm()),
        }
    }

    /// Render Markdown.
    pub fn render_markdown(content: &str, mut options: Options) -> Result<ByteString, StatusCode> {
        options.compile.allow_dangerous_html = true;
        to_html_with_options(content, &options).map(|html| html.into()).map_err_internal_server("Markdown to HTML")
    }
}

impl_resolve_from_str!(Renderer);
