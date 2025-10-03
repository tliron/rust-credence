use super::super::resolve::*;

use {
    compris::resolve::*,
    kutil::{cli::depict::*, http::*, std::metric::*},
};

//
// EncodingConfiguration
//

/// Encoding configuration.
#[derive(Clone, Debug, Depict, Resolve)]
pub struct EncodingConfiguration {
    /// Default.
    #[resolve]
    #[depict(style(symbol))]
    pub default: bool,

    /// Skip.
    #[resolve(key = "skip-media-types")]
    #[depict(iter(item), as(display), style(symbol))]
    pub skip_media_types: Vec<ResolveMediaType>,

    /// Minimum encodable body size.
    #[resolve(key = "min-body-size")]
    #[depict(as(display), style(symbol))]
    pub min_body_size: ResolveByteCount,
}

impl EncodingConfiguration {
    /// Skip media types.
    pub fn skip_media_types(&self) -> Vec<MediaType> {
        self.skip_media_types.iter().cloned().map(|media_type| media_type.inner.into()).collect()
    }
}

impl Default for EncodingConfiguration {
    fn default() -> Self {
        Self {
            default: true,
            skip_media_types: vec![
                MediaType::new_fostered("image", "png").into(),
                MediaType::new_fostered("image", "gif").into(),
                MediaType::new_fostered("image", "jpeg").into(),
                MediaType::new_fostered("audio", "mpeg").into(),
                MediaType::new_fostered("video", "mpeg").into(),
            ],
            min_body_size: ByteCount::from_kibibytes(1).into(),
        }
    }
}
