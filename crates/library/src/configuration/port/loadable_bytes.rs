use {
    bytes::*,
    bytestring::*,
    compris::resolve::*,
    kutil_std::error::*,
    std::{fmt, fs::*, io, path::*},
};

//
// LoadableBytes
//

/// A [Bytes] that be specified either explicitly or as a path from which to read it.
///
/// Resolves from:
/// * A single-key map where the key is either "content" or "path".
/// * Directly from a [Blob].
#[derive(Clone, Debug, Resolve)]
pub enum LoadableBytes {
    /// Content.
    #[resolve(single, key = "content")]
    Content(Bytes),

    /// Path.
    #[resolve(key = "path")]
    Path(PathBuf),
}

impl LoadableBytes {
    /// If it's [Content](LoadableBytes::Content), returns it.
    ///
    /// If it's [Path](LoadableBytes::Path), will attempt to read from it.
    pub fn to_bytes(&self) -> io::Result<Bytes> {
        match self {
            Self::Content(bytes) => Ok(bytes.clone()),
            Self::Path(path) => read(path).map(|bytes| bytes.into()).with_path(path),
        }
    }

    /// If it's already [Content](LoadableBytes::Content), returns self.
    ///
    /// If it's [Path](LoadableBytes::Path), will attempt to read from the path and return a new
    /// [LoadableBytes] with [Content](LoadableBytes::Content).
    pub fn into_content(self) -> io::Result<Self> {
        match &self {
            Self::Content(_) => Ok(self),
            Self::Path(_) => Ok(Self::Content(self.to_bytes()?)),
        }
    }
}

impl Default for LoadableBytes {
    fn default() -> Self {
        Self::Content(Bytes::default())
    }
}

impl From<Bytes> for LoadableBytes {
    fn from(blob: Bytes) -> Self {
        Self::Content(blob)
    }
}

impl From<ByteString> for LoadableBytes {
    fn from(string: ByteString) -> Self {
        Self::Content(string.into_bytes())
    }
}

impl From<&str> for LoadableBytes {
    fn from(string: &str) -> Self {
        Self::Content(Bytes::copy_from_slice(string.as_bytes()))
    }
}

impl TryInto<Bytes> for LoadableBytes {
    type Error = io::Error;

    fn try_into(self) -> Result<Bytes, Self::Error> {
        match self {
            Self::Content(bytes) => Ok(bytes),
            Self::Path(_) => self.to_bytes(),
        }
    }
}

impl fmt::Display for LoadableBytes {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Content(bytes) => write!(formatter, "content: {} bytes", bytes.len()),
            Self::Path(path) => write!(formatter, "path: {}", path.display()),
        }
    }
}
