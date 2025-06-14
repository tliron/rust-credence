use super::{super::util::*, caching::*, encoding::*, error::*, files::*, port::*, render::*, requests::*, urls::*};

use {
    compris::{normal::*, parse::*, resolve::*, *},
    kutil_cli::debug::*,
    kutil_http::{
        cache::{Cache, CacheKey},
        tower::caching::*,
        *,
    },
    std::{collections::*, io, path::*},
};

//
// CredenceConfiguration
//

/// Credence configuration.
#[derive(Clone, Debug, Debuggable, Resolve)]
pub struct CredenceConfiguration {
    /// Files.
    #[resolve]
    #[debuggable(as(debuggable))]
    pub files: FilesConfiguration,

    /// Ports.
    #[resolve]
    #[debuggable(iter(kv), key_style(number), as(debuggable))]
    pub ports: BTreeMap<u16, Port>,

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

    /// URLs.
    #[resolve]
    #[debuggable(as(debuggable))]
    pub urls: UrlsConfiguration,

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
        for port in &mut self.ports.values_mut() {
            port.validate(&base_path)?;
        }

        self.files.validate(base_path)
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
    pub fn hide(&self, uri_path: &str) -> bool {
        if uri_path_has_hidden_segment(uri_path) {
            return true;
        }

        for hide in &self.urls.hide {
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
        let asset_path = self.files.asset(uri_path);

        if let Some(base_file_name) = asset_path.file_name() {
            if let Some(parent) = asset_path.parent() {
                if parent.is_dir() {
                    let base_file_name = base_file_name.to_string_lossy().into_owned() + &self.render.midfix;
                    for file_path in parent.read_dir()? {
                        let file_path = file_path?.path();
                        if let Some(file_name) = file_path.file_name() {
                            let file_name = file_name.to_string_lossy();
                            if file_name.starts_with(&base_file_name) {
                                let extension = &file_name[base_file_name.len()..];
                                return Ok(Some(String::from(uri_path) + &self.render.midfix + extension));
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
        let mut port = Port::default();
        port.name = "http".into();

        Self {
            files: FilesConfiguration::default(),
            ports: BTreeMap::from([(8000, port)]),
            requests: RequestsConfiguration::default(),
            caching: CachingConfiguration::default(),
            encoding: EncodingConfiguration::default(),
            render: RenderConfiguration::default(),
            urls: UrlsConfiguration::default(),
        }
    }
}
