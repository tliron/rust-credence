use super::super::configuration::*;

use {
    axum::{http::StatusCode, response::*},
    bytestring::*,
    kutil_http::*,
    kutil_std::fs::*,
    minijinja::*,
    serde::*,
    std::{borrow::*, path::*},
};

//
// Templates
//

/// Templates.
#[derive(Clone, Debug)]
pub struct Templates {
    /// Jinja environment.
    pub jinja_environment: Environment<'static>,
}

impl Templates {
    /// Constructor.
    pub fn new(configuration: &PathsConfiguration) -> Self {
        let mut jinja_environment = Environment::new();
        jinja_environment.set_loader(path_loader_with_default(&configuration.templates));
        jinja_environment.set_keep_trailing_newline(true);
        jinja_environment.set_lstrip_blocks(true);
        jinja_environment.set_trim_blocks(true);

        minijinja_contrib::add_to_environment(&mut jinja_environment);

        jinja_environment.add_global("_assets_path", configuration.assets.to_string_lossy());
        jinja_environment.add_filter("version", version_filter);

        Self { jinja_environment }
    }

    /// Render template.
    pub async fn render<ContextT>(&self, template_name: &str, context: ContextT) -> Result<ByteString, StatusCode>
    where
        ContextT: Serialize,
    {
        let template = self.jinja_environment.get_template(template_name).map_err_internal_server("get template")?;

        template.render(context).map(|string| string.into()).map_err_internal_server("render template")
    }
}

fn path_loader_with_default<'path, PathT>(
    path: PathT,
) -> impl Fn(&str) -> Result<Option<String>, Error> + 'static + Send + Sync
where
    PathT: AsRef<Path> + 'path,
{
    let load = path_loader(path);
    move |path| {
        load(path).map(|result| {
            result.or_else(|| if path == DEFAULT_TEMPLATE { Some(DEFAULT_TEMPLATE_CONTENT.into()) } else { None })
        })
    }
}

fn version_filter(state: &State, value: Cow<'_, str>) -> String {
    if let Some(assets_path) = state.lookup("_assets_path") {
        if let Some(assets_path) = assets_path.as_str() {
            let assets_path: PathBuf = assets_path.into();
            let path = assets_path.join(value.as_ref());
            if let Ok(identifier) = file_modification_identifier(path) {
                return identifier.to_string();
            }
        }
    }

    "".into()
}

// See: https://stackoverflow.com/a/13416784/849021
const DEFAULT_TEMPLATE_CONTENT: &str = include_str!("../../../../assets/templates/default.jinja");
