use super::{
    super::{render::*, resolve::*, util::*},
    annotations::*,
    constants::*,
};

use {
    compris::{annotate::*, normal::*, resolve::*, *},
    kutil_cli::debug::*,
    kutil_std::{collections::*, metric::*, zerocopy::*},
};

//
// RenderConfiguration
//

/// Render configuration.
#[derive(Clone, Debug, Debuggable, Resolve)]
pub struct RenderConfiguration {
    /// Global variables.
    #[resolve]
    #[debuggable(iter(kv), as(debuggable), key_style(string))]
    pub variables: FastHashMap<ByteString, Variant<WithAnnotations>>,

    /// Rendered page URI midfix.
    #[resolve(key = "midfix")]
    #[debuggable(style(string))]
    pub midfix: ByteString,

    /// Annotations.
    #[resolve]
    #[debuggable(as(debuggable))]
    pub annotations: AnnotationsConfiguration,

    /// Default renderer.
    #[resolve(key = "default-renderer")]
    #[debuggable(as(display), style(symbol))]
    pub default_renderer: Renderer,

    /// Default template.
    #[resolve(key = "default-template")]
    #[debuggable(style(string))]
    pub default_template: ByteString,

    /// Maximum content size.
    #[resolve(key = "max-content-size")]
    #[debuggable(as(display), style(symbol))]
    pub max_content_size: ResolveByteCount,
}

impl RenderConfiguration {
    /// Whether the URI points to a rendered page, and if so returns its type.
    pub fn is_rendered_page(&self, uri_path: &str) -> Option<RenderedPageType> {
        if let Some(last_dot) = uri_path.rfind(EXTENSION_SEPARATOR) {
            let uri_path_without_extension = &uri_path[..last_dot];
            let rendered_page_midfix: &str = &self.midfix;
            if uri_path_without_extension.ends_with(rendered_page_midfix) {
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
            variables: FastHashMap::default(),
            annotations: Default::default(),
            midfix: ".r".into(),
            default_renderer: Default::default(),
            default_template: DEFAULT_TEMPLATE.into(),
            max_content_size: ByteCount::from_mebibytes(10).into(),
        }
    }
}
