use {
    axum::http::StatusCode,
    compris::*,
    kutil_http::*,
    kutil_std::string::*,
    markdown::*,
    std::{fmt, str::*},
};

//
// Renderer
//

/// Renderer.
#[derive(Clone, Copy, Debug, Default)]
pub enum Renderer {
    /// Passthrough.
    Passthrough,

    /// Markdown.
    Markdown,

    /// GitHub-Flavored Markdown.
    #[default]
    GFM,
}

impl Renderer {
    /// Render.
    pub async fn render(&self, content: &str) -> Result<String, StatusCode> {
        match self {
            Self::Passthrough => Ok(content.into()),
            Self::Markdown => Self::render_markdown(content, Options::default()),
            Self::GFM => Self::render_markdown(content, Options::gfm()),
        }
    }

    /// Render Markdown.
    pub fn render_markdown(content: &str, mut options: Options) -> Result<String, StatusCode> {
        options.compile.allow_dangerous_html = true;
        to_html_with_options(content, &options).map_err_internal_server("Markdown to HTML")
    }
}

impl FromStr for Renderer {
    type Err = ParseError;

    fn from_str(representation: &str) -> Result<Self, Self::Err> {
        match representation.to_lowercase().as_str() {
            "passthrough" => Ok(Self::Passthrough),
            "markdown" | "md" => Ok(Self::Markdown),
            "gfm" => Ok(Self::GFM),
            _ => Err(format!("unsupported: {}", representation).into()),
        }
    }
}

impl fmt::Display for Renderer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(
            match self {
                Self::Passthrough => "passthrough",
                Self::Markdown => "markdown",
                Self::GFM => "gfm",
            },
            formatter,
        )
    }
}

impl_resolve_from_str!(Renderer);
