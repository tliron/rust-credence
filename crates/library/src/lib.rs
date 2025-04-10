// https://stackoverflow.com/a/61417700
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(missing_docs)]

/*!
An opinionated little web server designed for easy maintenance and scalable performance.

For more information and usage examples see the
[home page](https://github.com/tliron/rust-credence).
*/

/// Configuration.
pub mod configuration;

/// File modification coordinator.
pub mod coordinator;

/// Axum middleware.
pub mod middleware;

/// Parse.
pub mod parse;

/// Render.
pub mod render;

/// Utilities.
pub mod util;
