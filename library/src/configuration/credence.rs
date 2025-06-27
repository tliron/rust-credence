use super::{
    super::{middleware::*, util::*},
    caching::*,
    encoding::*,
    error::*,
    files::*,
    port::*,
    render::*,
    requests::*,
    urls::*,
};

use {
    compris::{annotate::*, normal::*, parse::*, resolve::*, *},
    kutil_cli::debug::*,
    kutil_http::{
        cache::{Cache, CommonCacheKey},
        tower::caching::*,
        *,
    },
    kutil_std::zerocopy::*,
    std::{collections::*, io, path::*},
};

//
// CredenceConfiguration
//

/// Credence configuration.
#[derive(Clone, Debug, Debuggable, Resolve)]
pub struct CredenceConfiguration {
    /// Definitions (ignored).
    #[resolve]
    #[debuggable(skip)]
    pub definitions: Option<Variant<WithoutAnnotations>>,

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

    /// URLs.
    #[resolve]
    #[debuggable(as(debuggable))]
    pub urls: UrlsConfiguration,

    /// Render.
    #[resolve]
    #[debuggable(as(debuggable))]
    pub render: RenderConfiguration,

    /// Caching.
    #[resolve]
    #[debuggable(as(debuggable))]
    pub caching: CachingConfiguration,

    /// Encoding.
    #[resolve]
    #[debuggable(as(debuggable))]
    pub encoding: EncodingConfiguration,
}

impl CredenceConfiguration {
    /// Resolve.
    pub fn read<ReadT>(reader: &mut ReadT, source: ByteString) -> Result<Self, ConfigurationError>
    where
        ReadT: io::Read,
    {
        let variant = with_annotations!(
            Parser::new(Format::YAML)
                .with_source(source)
                .with_try_unsigned_integers(true)
                .parse(reader)
                .map_err(io::Error::other)?
        );

        let mut errors = ResolveErrors::default();
        let configuration = variant.resolve_with_errors(&mut errors).map_err(io::Error::other)?;
        if errors.is_empty() { Ok(configuration.ok_or(ConfigurationError::None)?) } else { Err(errors.into()) }
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
    pub fn caching_layer<RequestBodyT, CacheT>(
        &self,
        cache: CacheT,
    ) -> CachingLayer<RequestBodyT, CacheT, CommonCacheKey>
    where
        CacheT: Cache<CommonCacheKey>,
    {
        // For closure move
        let skip_media_types = self.encoding.skip_media_types();

        CachingLayer::default()
            .cache(cache.clone())
            .cacheable_by_default(self.caching.default)
            .cache_key(|context| {
                if let Some(socket) = context.request.extensions().get::<Socket>() {
                    context.cache_key.host = Some(socket.host.clone());
                }
            })
            .min_cacheable_body_size(self.caching.min_body_size.inner.into())
            .max_cacheable_body_size(self.caching.max_body_size.inner.into())
            .min_encodable_body_size(self.encoding.min_body_size.inner.into())
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
            if hide.inner.is_match(uri_path) {
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
        if let Some(base_file_name) = asset_path.file_name()
            && let Some(parent) = asset_path.parent()
            && parent.is_dir()
        {
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

        Ok(None)
    }
}

impl Default for CredenceConfiguration {
    fn default() -> Self {
        let mut port = Port::default();
        port.name = "http".into();

        Self {
            files: Default::default(),
            ports: BTreeMap::from([(8000, port)]),
            requests: Default::default(),
            caching: Default::default(),
            encoding: Default::default(),
            render: Default::default(),
            urls: Default::default(),
            definitions: None,
        }
    }
}
