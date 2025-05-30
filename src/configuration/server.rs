use super::{
    super::cli::*, caching::*, constants::*, encoding::*, listen::*, paths::*, render::*,
    requests::*, uri::*,
};

use {
    compris::{parse::Parser, resolve::*, *},
    kutil_http::{
        cache::{Cache, CacheKey},
        tower::caching::*,
        *,
    },
    std::{fs::*, io, path::*},
};

//
// ServerConfiguration
//

/// Server configuration.
#[derive(Clone, Debug, Resolve)]
pub struct ServerConfiguration {
    /// Paths.
    #[resolve]
    pub paths: PathsConfiguration,

    /// Listen.
    #[resolve]
    pub listen: Vec<Listen>,

    /// Requests.
    #[resolve]
    pub requests: RequestsConfiguration,

    /// Caching.
    #[resolve]
    pub caching: CachingConfiguration,

    /// Encoding.
    #[resolve]
    pub encoding: EncodingConfiguration,

    /// URI.
    #[resolve]
    pub uri: UriConfiguration,

    /// Render.
    #[resolve]
    pub render: RenderConfiguration,
}

impl ServerConfiguration {
    /// With base path.
    pub fn with_base_path<PathT>(&mut self, base_path: PathT)
    where
        PathT: AsRef<Path>,
    {
        for listen in &mut self.listen {
            listen.with_base_path(&base_path);
        }

        self.paths.with_base_path(base_path);
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
        for suffix in &self.uri.hide_suffixes {
            if uri_path.ends_with(suffix) {
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
            let base_file_name = base_file_name.to_string_lossy();
            if let Some(parent) = asset_path.parent() {
                for file_path in parent.read_dir()? {
                    let file_path = file_path?.path();
                    if let Some(file_name) = file_path.file_name() {
                        let file_name = file_name.to_string_lossy();
                        if file_name.starts_with(base_file_name.as_ref()) {
                            if let Some(last_dot) = file_name.rfind('.') {
                                let path_string_without_extension = &file_name[..last_dot];
                                if path_string_without_extension
                                    .ends_with(&self.render.rendered_page_midfix)
                                {
                                    let extension = &file_name
                                        [last_dot - self.render.rendered_page_midfix.len()..];
                                    return Ok(Some(String::from(uri_path) + extension));
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(None)
    }
}

impl Default for ServerConfiguration {
    fn default() -> Self {
        let mut listen = Listen::default();
        listen.name = "http".into();

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

impl CLI {
    /// Server configuration.
    pub fn server_configuration(&self) -> io::Result<ServerConfiguration> {
        let assets_path = self.path.canonicalize()?;
        let configuration_base_path = assets_path.join(CREDENCE_SUFFIX);
        let configuration_path = configuration_base_path.join(SERVER_CONFIGURATION_FILE);

        let mut configuration = if configuration_path.exists() {
            let value = Parser::new(Format::YAML)
                .with_try_unsigned_integers(true)
                .parse(&mut File::open(configuration_path)?)
                .map_err(io::Error::other)?;

            <normal::Value as Resolve<_, CommonResolveContext, CommonResolveError>>::resolve(&value)
                .map_err(io::Error::other)?
                .ok_or(io::Error::other("no configuration"))?
        } else {
            tracing::info!("configuration not found: {}", configuration_path.display());
            ServerConfiguration::default()
        };

        configuration.paths.with_assets_path(assets_path);
        configuration.with_base_path(&configuration_base_path);

        Ok(configuration)
    }
}
