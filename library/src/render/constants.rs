use kutil::http::*;

/// Index.
pub const INDEX: &str = "index";

/// `text/html`.
pub const HTML_MEDIA_TYPE_STRING: &str = "text/html";

/// `application/json`.
pub const JSON_MEDIA_TYPE_STRING: &str = "application/json";

/// `text/html` [MediaType].
pub const HTML_MEDIA_TYPE: MediaType = MediaType::new_fostered("text", "html");

/// `application/json` [MediaType].
pub const JSON_MEDIA_TYPE: MediaType = MediaType::new_fostered("application", "json");

/// Supported renderable [MediaTypeSelector]s.
pub const RENDERED_PAGE_MEDIA_TYPES: &[MediaTypeSelector] =
    &[MediaTypeSelector::new_fostered("text", "html"), MediaTypeSelector::new_fostered("application", "json")];
