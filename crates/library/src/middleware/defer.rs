use {
    axum::{
        extract::*,
        http::{StatusCode, header::*},
        response::*,
    },
    bytestring::*,
};

//
// DeferredResponse
//

/// Deferred response as a [Request] extension.
#[derive(Clone, Debug)]
pub enum DeferredResponse {
    /// Hide.
    Hide,

    /// Authenticate.
    Authenticate(ByteString),

    /// Redirect to URI path.
    RedirectTo(ByteString),

    /// Rewrite from URI path.
    RewriteFrom(ByteString),

    /// Error.
    Error(ByteString),
}

impl DeferredResponse {
    /// Get.
    pub fn get(request: &Request) -> Option<&Self> {
        request.extensions().get()
    }

    /// Authentication response.
    ///
    /// [StatusCode::UNAUTHORIZED] with a `WWW-Authenticate` header value.
    pub fn authenticate(authenticate: &str) -> Response {
        (StatusCode::UNAUTHORIZED, [(WWW_AUTHENTICATE, authenticate)]).into_response()
    }

    /// Redirection response.
    ///
    /// [StatusCode::MOVED_PERMANENTLY] with a `Location` header value.
    pub fn redirect_to(uri_path: &str) -> Response {
        (StatusCode::MOVED_PERMANENTLY, [(LOCATION, uri_path)]).into_response()
    }
}

//
// WithDeferredResponse
//

/// With [DeferredResponse].
pub trait WithDeferredResponse {
    /// With [DeferredResponse].
    fn with_deferred_response(self, delayed_response: DeferredResponse) -> Self;

    /// With [DeferredResponse::Hide].
    fn with_deferred_hide(self) -> Self;

    /// With [DeferredResponse::Authenticate].
    fn with_deferred_authenticate(self, authenticate: ByteString) -> Self;

    /// With [DeferredResponse::RedirectTo].
    fn with_deferred_redirect_to(self, uri_path: ByteString) -> Self;

    /// With [DeferredResponse::RewriteFrom].
    fn with_deferred_rewrite_from(self, uri_path: ByteString) -> Self;

    /// With [DeferredResponse::Error].
    fn with_deferred_error(self, error: ByteString) -> Self;
}

impl WithDeferredResponse for Request {
    fn with_deferred_response(mut self, delayed_response: DeferredResponse) -> Self {
        self.extensions_mut().insert(delayed_response);
        self
    }

    fn with_deferred_hide(self) -> Self {
        self.with_deferred_response(DeferredResponse::Hide)
    }

    fn with_deferred_authenticate(self, authenticate: ByteString) -> Self {
        self.with_deferred_response(DeferredResponse::Authenticate(authenticate))
    }

    fn with_deferred_redirect_to(self, uri_path: ByteString) -> Self {
        self.with_deferred_response(DeferredResponse::RedirectTo(uri_path))
    }

    fn with_deferred_rewrite_from(self, uri_path: ByteString) -> Self {
        self.with_deferred_response(DeferredResponse::RewriteFrom(uri_path))
    }

    fn with_deferred_error(self, error: ByteString) -> Self {
        self.with_deferred_response(DeferredResponse::Error(error))
    }
}
