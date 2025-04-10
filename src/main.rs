// https://stackoverflow.com/a/61417700
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(missing_docs)]

/*!
An opinionated little web server designed for easy maintenance and scalable performance.

For more information and usage examples see the
[home page](https://github.com/tliron/rust-credence).
*/

mod cli;
mod configuration;
mod errors;
mod middleware;
mod parse;
mod render;
mod router;
mod run;
mod serve;

use run::*;

use {mimalloc::*, std::process::*};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

/// Main.
pub fn main() -> ExitCode {
    kutil_cli::run::run(run)
}
