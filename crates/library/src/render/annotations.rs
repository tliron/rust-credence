use super::{
    super::{configuration::*, parse::*},
    renderer::*,
};

use {
    bytestring::*,
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
    pub created: Option<ResolveDateTime>,

    /// Last updated.
    #[resolve]
    pub updated: Option<ResolveDateTime>,

    /// Renderer.
    #[resolve]
    pub renderer: Option<Renderer>,

    /// Template.
    #[resolve]
    pub template: Option<ByteString>,

    /// Values.
    #[resolve]
    pub values: FastHashMap<ByteString, Value>,

    /// Headers.
    #[resolve]
    pub headers: FastHashMap<ByteString, ByteString>,

    /// Other.
    #[resolve(other_keys)]
    pub other: FastHashMap<ByteString, Value>,
}

impl Annotations {
    /// Renderer.
    pub fn renderer<'own>(&'own self, configuration: &'own RenderConfiguration) -> &'own Renderer {
        self.renderer.as_ref().unwrap_or(&configuration.default_renderer)
    }

    /// Template.
    pub fn template<'own>(&'own self, configuration: &'own RenderConfiguration) -> &'own str {
        self.template.as_ref().unwrap_or(&configuration.default_template)
    }

    /// Parse.
    pub fn parse(representation: &str, format: Format) -> Self {
        match Parser::new(format).parse_from_string(representation) {
            Ok(annotations) => match Resolve::<_, CommonResolveContext, CommonResolveError>::resolve(&annotations) {
                Ok(annotations) => {
                    if let Some(annotations) = annotations {
                        return annotations;
                    }
                }

                Err(error) => tracing::error!("{}", error),
            },

            Err(error) => tracing::error!("{}", error),
        }

        Self::default()
    }

    /// Traverse value.
    pub fn traverse_value(&self, keys: &RefValuePath<'_>) -> Option<&Value> {
        if !keys.is_empty() {
            if let Value::Text(first_key) = &keys[0] {
                if let Some(first_value) = self.values.get(&first_key.value) {
                    let keys = keys[1..].iter().map(|value| *value);
                    return first_value.traverse(keys);
                }
            }
        }

        None
    }
}

impl AnnotationsConfiguration {
    /// Split [Annotations] from content.
    pub fn split<'content>(&self, content: &'content str) -> (Annotations, &'content str) {
        let start_delimiter: &str = &self.start_delimiter;
        if content.starts_with(start_delimiter) {
            let end_delimiter: &str = &self.end_delimiter;
            if let Some((properties, content)) =
                content[start_delimiter.len()..].split_once_ignore_escaped(end_delimiter)
            {
                // Unescape end delimiter
                let annotations = properties.unescape(end_delimiter);
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
