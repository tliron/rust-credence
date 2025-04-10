use super::{super::super::configuration::*, filters::*};

use {
    ::axum::http::*,
    bytestring::*,
    kutil_http::*,
    minijinja::{Error, *},
    serde::*,
    std::{path::*, result::Result},
};

const DEFAULT_TEMPLATE_CONTENT: &str = include_str!("default.jinja");

/// Default [DateTime] format.
pub const DEFAULT_DATE_TIME_FORMAT: &str = "[month repr:long] [day padding:none], [year]";

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
    pub fn new(configuration: &FilesConfiguration) -> Self {
        let mut jinja_environment = Environment::new();
        jinja_environment.set_loader(path_loader_with_default(&configuration.templates));
        jinja_environment.set_keep_trailing_newline(true);
        jinja_environment.set_lstrip_blocks(true);
        jinja_environment.set_trim_blocks(true);

        jinja_environment.add_global("DATE_FORMAT", DEFAULT_DATE_TIME_FORMAT);
        minijinja_contrib::add_to_environment(&mut jinja_environment);

        jinja_environment.add_global("ASSETS_PATH", configuration.assets.to_string_lossy());
        jinja_environment.add_filter("fileversion", fileversion_filter);

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
    let loader = path_loader(path);
    move |path| {
        loader(path).map(|template| {
            template.or_else(|| if path == DEFAULT_TEMPLATE { Some(DEFAULT_TEMPLATE_CONTENT.into()) } else { None })
        })
    }
}
