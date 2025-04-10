use super::super::configuration::*;

use {
    axum::{http::StatusCode, response::*},
    kutil_http::*,
    minijinja::*,
    serde::*,
    std::path::*,
};

//
// Templates
//

/// Templates.
#[derive(Clone, Debug)]
pub struct Templates {
    environment: Environment<'static>,
}

impl Templates {
    /// Constructor.
    pub fn new<PathT>(base_path: PathT) -> Self
    where
        PathT: AsRef<Path>,
    {
        let mut environment = Environment::new();
        environment.set_loader(path_loader_with_default(base_path));
        environment.set_keep_trailing_newline(true);
        environment.set_lstrip_blocks(true);
        environment.set_trim_blocks(true);

        Self { environment }
    }

    /// Render template.
    pub async fn render<ContextT>(
        &self,
        template_name: &str,
        values: ContextT,
    ) -> Result<Vec<u8>, StatusCode>
    where
        ContextT: Serialize,
    {
        let template = self
            .environment
            .get_template(&template_name)
            .map_err_internal_server("get template")?;

        template
            .render(values)
            .map(|string| string.into())
            .map_err_internal_server("render template")
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
            result.or_else(|| {
                if path == DEFAULT_TEMPLATE {
                    Some(DEFAULT_TEMPLATE_CONTENT.into())
                } else {
                    None
                }
            })
        })
    }
}

// See: https://stackoverflow.com/a/13416784/849021
const DEFAULT_TEMPLATE_CONTENT: &str = include_str!("../../assets/templates/default.html");
