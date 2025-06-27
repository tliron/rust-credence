// https://stackoverflow.com/a/61417700
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(missing_docs)]

/*!
An unfussy web server designed for straightforward authoring and scalable performance.

For more information and usage examples see the
[home page](https://github.com/tliron/rust-credence).
*/

/// Configuration.
pub mod configuration;

/// File modification coordinator.
pub mod coordinator;

/// Axum middleware.
pub mod middleware;

/// Compris resolvers.
pub mod resolve;

/// Render.
pub mod render;

/// Server.
pub mod server;

/// Utilities.
pub mod util;
