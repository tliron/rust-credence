use {
    bytes::*,
    bytestring::*,
    compris::{normal::*, resolve::*},
    kutil_std::error::*,
    std::{fs::*, io, path::*},
};

//
// LoadableBytes
//

/// A [Bytes] that be specified either explicitly or as a path from which to read it.
///
/// Resolves from:
/// * A single-key map where the key is either "content" or "path".
/// * Directly from a [Blob].
#[derive(Clone, Debug)]
pub enum LoadableBytes {
    /// Bytes.
    Bytes(Bytes),

    /// Path.
    Path(PathBuf),
}

impl LoadableBytes {
    /// If it's [Bytes](LoadableBytes::Bytes), returns it.
    ///
    /// If it's [Path](LoadableBytes::Path), will attempt to read from it.
    pub fn as_bytes(self) -> io::Result<Bytes> {
        match self {
            Self::Bytes(bytes) => Ok(bytes),
            Self::Path(path) => read(path).map(|bytes| bytes.into()),
        }
    }

    /// If it's already [Bytes](LoadableBytes::Bytes), returns self.
    ///
    /// If it's [Path](LoadableBytes::Path), will attempt to read from the path and return a new
    /// [LoadableBytes] with [Bytes](LoadableBytes::Bytes).
    pub fn into_bytes(self) -> io::Result<Self> {
        match &self {
            Self::Bytes(_) => Ok(self),
            Self::Path(_) => Ok(Self::Bytes(self.as_bytes()?)),
        }
    }
}

impl Default for LoadableBytes {
    fn default() -> Self {
        Self::Bytes(Bytes::default())
    }
}

impl From<Bytes> for LoadableBytes {
    fn from(blob: Bytes) -> Self {
        Self::Bytes(blob)
    }
}

impl From<ByteString> for LoadableBytes {
    fn from(string: ByteString) -> Self {
        Self::Bytes(string.into_bytes())
    }
}

impl From<&str> for LoadableBytes {
    fn from(string: &str) -> Self {
        Self::Bytes(Bytes::copy_from_slice(string.as_bytes()))
    }
}

impl TryInto<Bytes> for LoadableBytes {
    type Error = io::Error;

    fn try_into(self) -> Result<Bytes, Self::Error> {
        self.as_bytes()
    }
}

impl<ContextT, ErrorT> Resolve<LoadableBytes, ContextT, ErrorT> for Value
where
    ContextT: ResolveContext,
    ErrorT: ResolveError,
{
    fn resolve_for<'own, ErrorRecipientT>(
        &'own self,
        context: Option<&ContextT>,
        mut ancestor: Option<&'own Value>,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<LoadableBytes, ErrorT>
    where
        ErrorRecipientT: ErrorRecipient<ErrorT>,
    {
        if ancestor.is_none() {
            ancestor = Some(self)
        }

        Ok(match self {
            Self::Blob(blob) => Some(blob.value.clone().into()),

            Self::Text(text) => Some(text.value.clone().into()),

            Self::Map(_) => match self.to_key_value_pair() {
                Some((key, value)) => match key {
                    Self::Text(text) => match text.as_str() {
                        "content" => Resolve::resolve_for(value, context, ancestor, errors)?.map(LoadableBytes::Bytes),

                        "path" => Resolve::resolve_for(value, context, ancestor, errors)?.map(LoadableBytes::Path),

                        key => {
                            errors.give(
                                MalformedError::new(
                                    "LoadableBytes",
                                    &format!("key is not \"content\" or \"path\": {}", key),
                                )
                                .with_citation_for(self, context, ancestor),
                            )?;
                            None
                        }
                    },

                    _ => {
                        errors.give(
                            IncompatibleValueTypeError::new(self, &["text"]).with_citation_for(self, context, ancestor),
                        )?;
                        None
                    }
                },

                None => {
                    errors.give(
                        MalformedError::new("map", "is not a single-key map")
                            .with_citation_for(self, context, ancestor),
                    )?;
                    None
                }
            },

            _ => {
                errors.give(
                    IncompatibleValueTypeError::new(self, &["blob", "text", "map"])
                        .with_citation_for(self, context, ancestor),
                )?;
                None
            }
        })
    }
}
