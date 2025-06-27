use super::context::*;

use {axum::http::*, std::result::Result};

//
// RenderPreparer
//

/// [RenderedPage] preparer.
#[allow(async_fn_in_trait)]
pub trait RenderPreparer {
    /// Prepare.
    async fn prepare<'own>(&self, context: &mut RenderContext<'own>) -> Result<(), StatusCode>;
}
