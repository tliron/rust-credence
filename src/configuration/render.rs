use super::{super::render::*, annotations::*, constants::*};

use {
    compris::{resolve::*, *},
    kutil_std::metric::*,
};

//
// RenderConfiguration
//

/// Render configuration.
#[derive(Clone, Debug, Resolve)]
pub struct RenderConfiguration {
    /// Rendered page midfix.
    #[resolve]
    #[resolve(key = "rendered-page-midfix")]
    pub rendered_page_midfix: String,

    /// Annotations.
    #[resolve]
    pub annotations: AnnotationsConfiguration,

    /// Default renderer.
    #[resolve(key = "default-renderer")]
    pub default_renderer: Renderer,

    /// Default template.
    #[resolve(key = "default-template")]
    pub default_template: String,

    /// Maximum content size.
    #[resolve(key = "max-content-size")]
    pub max_content_size: ResolveFromStr<ByteCount>,
}

impl RenderConfiguration {
    /// Whether the URI points to a rendered page, and if so returns its type.
    pub fn is_rendered_page(&self, uri_path: &str) -> Option<RenderedPageType> {
        if let Some(last_dot) = uri_path.rfind('.') {
            let uri_path_without_extension = &uri_path[..last_dot];
            if uri_path_without_extension.ends_with(&self.rendered_page_midfix) {
                let extension = &uri_path[last_dot + 1..];

                let format_result: Result<Format, _> = extension.parse();
                return Some(match format_result {
                    Ok(format) => RenderedPageType::Annotations(format),
                    Err(_) => RenderedPageType::ContentWithEmbeddedAnnotations,
                });
            }
        }

        None
    }
}

impl Default for RenderConfiguration {
    fn default() -> Self {
        Self {
            annotations: AnnotationsConfiguration::default(),
            rendered_page_midfix: DEFAULT_RENDER_MIDFIX.into(),
            default_renderer: Renderer::default(),
            default_template: DEFAULT_TEMPLATE.into(),
            max_content_size: ByteCount::from_mebibytes(1).into(),
        }
    }
}
