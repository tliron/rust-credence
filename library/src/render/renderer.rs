use super::{context::*, preparer::*};

use {
    ::axum::http::*,
    compris::*,
    kutil_http::*,
    kutil_std::{zerocopy::*, *},
    markdown::{mdast::*, *},
    std::result::Result,
};

//
// Renderer
//

/// Renderer.
#[derive(Clone, Copy, Debug, Default, Display, FromStr, Eq, Hash, PartialEq)]
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
            Self::Markdown => Self::render_markdown(content, Default::default()),
            Self::GFM => Self::render_markdown(content, Options::gfm()),
        }
    }

    /// Title from content.
    pub fn title_from_content(&self, content: &str) -> Result<Option<ByteString>, StatusCode> {
        match self {
            Self::Passthrough => Ok(self.title_from_passthrough(content)?.map(|title| title.into())),
            Self::Markdown => self.title_from_markdown(content, &Default::default()),
            Self::GFM => self.title_from_markdown(content, &ParseOptions::gfm()),
        }
    }

    /// Render Markdown.
    pub fn render_markdown(content: &str, mut options: Options) -> Result<ByteString, StatusCode> {
        options.compile.allow_dangerous_html = true;
        to_html_with_options(content, &options).map(|html| html.into()).map_err_internal_server("Markdown to HTML")
    }

    /// Title from passthrough content.
    ///
    /// This will be the first `<h1>`.
    pub fn title_from_passthrough<'content>(
        &self,
        content: &'content str,
    ) -> Result<Option<&'content str>, StatusCode> {
        if let Some(heading_start) = content.find("<h1>") {
            let title = &content[heading_start + 4..];
            if let Some(heading_end) = title.find("</h1>") {
                let title = &title[..heading_end];
                return Ok(Some(title));
            }
        }

        Ok(None)
    }

    /// Title from Markdown content.
    ///
    /// This will be the text in the first heading.
    pub fn title_from_markdown(&self, content: &str, options: &ParseOptions) -> Result<Option<ByteString>, StatusCode> {
        // TODO: other Markdown parsers might be more efficient here
        // allowing us *not* to parse the entire content in order to get the first heading

        let node = to_mdast(content, options).map_err_internal_server("parse Markdown")?;

        if let Some(children) = node.children() {
            for child in children {
                if let Node::Heading(heading) = child
                    && let Some(child) = heading.children.get(0)
                    && let Node::Text(text) = child
                {
                    return Ok(Some(text.value.clone().into()));
                }
            }
        }

        Ok(None)
    }
}

impl_resolve_from_str!(Renderer);

impl RenderPreparer for Renderer {
    async fn prepare<'own>(&self, context: &mut RenderContext<'own>) -> Result<(), StatusCode> {
        if !context.variables.contains_key("content") {
            if let Some(content) = context.rendered_page.content.as_ref() {
                let content = self.render(content).await?;
                context.variables.insert("content".into(), content.into());
            }
        }

        Ok(())
    }
}
