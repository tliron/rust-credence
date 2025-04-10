use crate::configuration::ConfigurationError;

use super::{
    super::util::*, caching::*, constants::*, encoding::*, listen::*, paths::*, render::*, requests::*, uri::*,
};

use {
    compris::{normal::*, parse::*, resolve::*, *},
    kutil_cli::debug::*,
    kutil_http::{
        cache::{Cache, CacheKey},
        tower::caching::*,
        *,
    },
    std::{io, path::*},
};

//
// CredenceConfiguration
//

/// Credence configuration.
#[derive(Clone, Debug, Debuggable, Resolve)]
pub struct CredenceConfiguration {
    /// Paths.
    #[resolve]
    #[debuggable(as(debuggable))]
    pub paths: PathsConfiguration,

    /// Listen.
    #[resolve]
    #[debuggable(iter(item), as(debuggable))]
    pub listen: Vec<Listen>,

    /// Requests.
    #[resolve]
    #[debuggable(as(debuggable))]
    pub requests: RequestsConfiguration,

    /// Caching.
    #[resolve]
    #[debuggable(as(debuggable))]
    pub caching: CachingConfiguration,

    /// Encoding.
    #[resolve]
    #[debuggable(as(debuggable))]
    pub encoding: EncodingConfiguration,

    /// URI.
    #[resolve]
    #[debuggable(as(debuggable))]
    pub uri: UriConfiguration,

    /// Render.
    #[resolve]
    #[debuggable(as(debuggable))]
    pub render: RenderConfiguration,
}

impl CredenceConfiguration {
    /// Resolve.
    pub fn read<ReadT>(reader: &mut ReadT) -> io::Result<Self>
    where
        ReadT: io::Read,
    {
        let value =
            Parser::new(Format::YAML).with_try_unsigned_integers(true).parse(reader).map_err(io::Error::other)?;

        <Value as Resolve<_, CommonResolveContext, CommonResolveError>>::resolve(&value)
            .map_err(io::Error::other)?
            .ok_or(io::Error::other("no configuration"))
    }

    /// Validate.
    pub fn validate<PathT>(&mut self, base_path: PathT) -> Result<(), ConfigurationError>
    where
        PathT: AsRef<Path>,
    {
        for listen in &mut self.listen {
            listen.validate(&base_path)?;
        }

        self.paths.validate(base_path)
    }

    /// Caching layer
    pub fn caching_layer<CacheT, CacheKeyT>(&self, cache: CacheT) -> CachingLayer<CacheT, CacheKeyT>
    where
        CacheT: Cache<CacheKeyT>,
        CacheKeyT: CacheKey,
    {
        // For closure move
        let skip_media_types = self.encoding.skip_media_types();

        CachingLayer::new()
            .cache(cache.clone())
            .cacheable_by_default(self.caching.default)
            .min_cacheable_body_size(self.caching.min_body_size.value.into())
            .max_cacheable_body_size(self.caching.max_body_size.value.into())
            .min_encodable_body_size(self.encoding.min_body_size.value.into())
            .encodable_by_default(self.encoding.default)
            .encodable_by_response(move |context| match context.headers.content_type() {
                Some(content_type) => !skip_media_types.contains(&content_type),
                None => true,
            })
    }

    /// Whether the URI path is hidden.
    pub fn is_hidden(&self, uri_path: &str) -> bool {
        if uri_path_has_hidden_segment(uri_path) {
            return true;
        }

        for hide in &self.uri.hide {
            //tracing::debug!("{} {}", hide.value, hide.value.is_match(uri_path));
            if hide.value.is_match(uri_path) {
                return true;
            }
        }

        self.render.is_rendered_page(uri_path).is_some()
    }

    /// Rendered page URI path.
    ///
    /// "{path}" -> "{path}.r.yaml" or "{path}.r.*"
    pub fn rendered_page_uri_path(&self, uri_path: &str) -> io::Result<Option<String>> {
        let asset_path = self.paths.asset(uri_path);

        if let Some(base_file_name) = asset_path.file_name() {
            if let Some(parent) = asset_path.parent() {
                if parent.is_dir() {
                    let base_file_name = base_file_name.to_string_lossy().into_owned() + &self.render.uri_midfix;
                    for file_path in parent.read_dir()? {
                        let file_path = file_path?.path();
                        if let Some(file_name) = file_path.file_name() {
                            let file_name = file_name.to_string_lossy();
                            if file_name.starts_with(&base_file_name) {
                                let extension = &file_name[base_file_name.len()..];
                                return Ok(Some(String::from(uri_path) + &self.render.uri_midfix + extension));
                            }
                        }
                    }
                }
            }
        }

        Ok(None)
    }
}

impl Default for CredenceConfiguration {
    fn default() -> Self {
        let mut listen = Listen::default();
        listen.name = DEFAULT_LISTEN_NAME.into();

        Self {
            paths: PathsConfiguration::default(),
            listen: vec![listen],
            requests: RequestsConfiguration::default(),
            caching: CachingConfiguration::default(),
            encoding: EncodingConfiguration::default(),
            render: RenderConfiguration::default(),
            uri: UriConfiguration::default(),
        }
    }
}
