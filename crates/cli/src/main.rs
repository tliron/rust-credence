// https://stackoverflow.com/a/61417700
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(missing_docs)]

/*!
An unfussy web server designed for straightforward authoring and scalable performance.

For more information and usage examples see the
[home page](https://github.com/tliron/rust-credence).
*/

mod cli;
mod errors;
mod run;
mod start;

use run::*;

use {mimalloc::*, std::process::*};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

/// Main.
pub fn main() -> ExitCode {
    kutil_cli::run::run(run)
}
