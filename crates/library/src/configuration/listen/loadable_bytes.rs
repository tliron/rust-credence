use {
    bytes::*,
    bytestring::*,
    compris::resolve::*,
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
    /// Bytes.
    #[resolve(single, key = "content")]
    Bytes(Bytes),

    /// Path.
    #[resolve(key = "path")]
    Path(PathBuf),
}

impl LoadableBytes {
    /// If it's [Bytes](LoadableBytes::Bytes), returns it.
    ///
    /// If it's [Path](LoadableBytes::Path), will attempt to read from it.
    pub fn to_bytes(&self) -> io::Result<Bytes> {
        match self {
            Self::Bytes(bytes) => Ok(bytes.clone()),
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
            Self::Path(_) => Ok(Self::Bytes(self.to_bytes()?)),
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
        self.to_bytes()
    }
}

impl fmt::Display for LoadableBytes {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bytes(bytes) => write!(formatter, "{} bytes", bytes.len()),
            Self::Path(path_buf) => write!(formatter, "path = {}", path_buf.display()),
        }
    }
}
