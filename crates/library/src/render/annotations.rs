use super::{
    super::{configuration::*, resolve::*},
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

    /// Variables.
    #[resolve]
    pub variables: FastHashMap<ByteString, Value>,

    /// Headers.
    #[resolve]
    pub headers: FastHashMap<ByteString, ByteString>,

    /// Other.
    #[resolve(other_keys)]
    pub other: FastHashMap<ByteString, Value>,
}

impl Annotations {
    /// Renderer.
    pub fn renderer(&self, configuration: &RenderConfiguration) -> Renderer {
        self.renderer.clone().unwrap_or(configuration.default_renderer)
    }

    /// Template.
    pub fn template<'own>(&'own self, configuration: &'own RenderConfiguration) -> &'own str {
        self.template.as_ref().unwrap_or(&configuration.default_template)
    }

    /// Parse.
    pub fn parse(identifier: &str, representation: &str, format: Format) -> Self {
        match Parser::new(format).with_try_unsigned_integers(true).parse_from_string(representation) {
            Ok(annotations) => match Resolve::<_, CommonResolveContext, CommonResolveError>::resolve(&annotations) {
                Ok(annotations) => {
                    if let Some(annotations) = annotations {
                        return annotations;
                    }
                }

                Err(error) => tracing::error!("{}: {}", identifier, error),
            },

            Err(error) => tracing::error!("{}: {}", identifier, error),
        }

        Self::default()
    }

    /// Traverse variable.
    pub fn traverse_variable(&self, keys: &RefValuePath<'_>) -> Option<&Value> {
        if !keys.is_empty() {
            if let Value::Text(first_key) = &keys[0] {
                if let Some(first_value) = self.variables.get(&first_key.value) {
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
    pub fn split<'content>(&self, identifier: &str, content: &'content str) -> (Annotations, &'content str) {
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
                    match annotations.split_once('\n') {
                        Some((format, annotations_after_format)) => {
                            let format_result = format.parse();
                            annotations = annotations_after_format;
                            format_result.unwrap_or(self.default_format.value.into())
                        }

                        None => self.default_format.value.into(),
                    }
                };

                let annotations = Annotations::parse(identifier, annotations, format);
                return (annotations, content);
            }
        }

        (Annotations::default(), content)
    }
}
