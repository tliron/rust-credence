use super::context::*;

use {async_trait::*, axum::http::StatusCode};

/// Render add-on.
#[async_trait]
pub trait RenderHandler {
    /// Render.
    async fn render<'own>(&self, context: &mut RenderContext<'own>) -> Result<(), StatusCode>;
}

fn _make() -> Box<dyn RenderHandler> {
    unimplemented!()
}
