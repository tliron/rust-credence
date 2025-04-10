use super::{
    super::{configuration::*, parse::*},
    renderer::*,
};

use {
    chrono::*,
    compris::{normal::*, parse::Parser, resolve::*, *},
    kutil_std::{collections::*, string::*},
};

//
// Annotations
//

/// Annotations.
#[derive(Clone, Debug, Default, Resolve)]
pub struct Annotations {
    /// Created.
    #[resolve]
    pub created: Option<ResolveParseStr<DateTime<Utc>, ParseDateTime>>,

    /// Updated.
    #[resolve]
    pub updated: Option<ResolveParseStr<DateTime<Utc>, ParseDateTime>>,

    /// Renderer.
    #[resolve]
    pub renderer: Option<Renderer>,

    /// Template.
    #[resolve]
    pub template: Option<String>,

    /// Values.
    #[resolve]
    pub values: FastHashMap<String, Value>,

    /// Headers.
    #[resolve]
    pub headers: FastHashMap<String, String>,

    /// Other.
    #[resolve(other_keys)]
    pub other: FastHashMap<String, Value>,
}

impl Annotations {
    /// Renderer.
    pub fn renderer<'own>(&'own self, configuration: &'own RenderConfiguration) -> &'own Renderer {
        self.renderer
            .as_ref()
            .unwrap_or(&configuration.default_renderer)
    }

    /// Template.
    pub fn template<'own>(&'own self, configuration: &'own RenderConfiguration) -> &'own str {
        self.template
            .as_ref()
            .unwrap_or(&configuration.default_template)
    }

    /// Parse.
    pub fn parse(representation: &str, format: Format) -> Self {
        match Parser::new(format).parse_from_string(representation) {
            Ok(annotations) => {
                match Resolve::<_, CommonResolveContext, CommonResolveError>::resolve(&annotations)
                {
                    Ok(annotations) => {
                        if let Some(annotations) = annotations {
                            return annotations;
                        }
                    }

                    Err(error) => tracing::error!("{}", error),
                }
            }

            Err(error) => tracing::error!("{}", error),
        }

        Self::default()
    }
}

impl AnnotationsConfiguration {
    /// Split [Annotations] from content.
    pub fn split<'content>(&self, content: &'content str) -> (Annotations, &'content str) {
        if content.starts_with(&self.start_delimiter) {
            if let Some((properties, content)) =
                content[self.start_delimiter.len()..].split_once_ignore_escaped(&self.end_delimiter)
            {
                // Unescape end delimiter
                let annotations = properties.unescape(&self.end_delimiter);
                let mut annotations = annotations.as_str();

                let format = {
                    // The format can be right after the start delimiter with a newline
                    match annotations.split_once("\n") {
                        Some((format, annotations_after_format)) => {
                            let format_result = format.parse();
                            annotations = annotations_after_format;
                            format_result.unwrap_or(self.default_format.value.into())
                        }

                        None => self.default_format.value.into(),
                    }
                };

                let annotations = Annotations::parse(annotations, format);
                return (annotations, content);
            }
        }

        (Annotations::default(), content)
    }
}
