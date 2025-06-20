[![crates.io](https://img.shields.io/crates/v/credence?color=%23227700)](https://crates.io/crates/credence-lib)
[![docs.rs](https://img.shields.io/badge/docs.rs-latest?color=grey)](https://docs.rs/credence-lib/latest/credence_lib/)

Credence
========

An unfussy web server designed for straightforward authoring and scalable performance.

Suitable for hosting blogs, newsletters, personal sites, documentation, etc.

Do you like databases, frameworks, "stacks", buzzwords, and bloat? Do you know what a "CMS" is? Then Credence is probably not for you. Welcome to the nonsense-free web zone.

Rustaceans: All of Credence's functionality lives in [credence-lib](https://crates.io/crates/credence-lib), so you can enhance it, integrate it into something else, remix it, etc.

Documentation
-------------

* The [basics guide](https://github.com/tliron/rust-credence/blob/main/documentation/basics.md) will get you up and running with the default configuration.
* The [advanced guide](https://github.com/tliron/rust-credence/blob/main/documentation/advanced.md) will take it to the next level.
* We also have some useful [tips](https://github.com/tliron/rust-credence/blob/main/documentation/tips.md)!
* And for Rustaceans there's the [credence-lib documentation](https://docs.rs/credence-lib/latest/credence_lib/).

Features
--------

### Usability

* Distributed as a single executable
* Just point it at your web assets directories
* Sensible out-of-the-box defaults
* Straightforward configuration in YAML if you need to go beyond the defaults
* Serve multiple sites from a single Credence process
* A reverse proxy is *not* required: Credence aims to have most everything you should need for outward exposure

### Authoring

* Your file directory structure = your URL structure
* Write your content in [Markdown](https://en.wikipedia.org/wiki/Markdown) (with a few optional annotations)
* Design your HTML in ... HTML! (with optional [Jinja](https://en.wikipedia.org/wiki/Jinja_\(template_engine\)) conveniences)
* Turn a directory full of Markdown pages into a user-searchable catalog
* Every rendered page has a JSON endpoint (for your fancy JavaScript widgets)
* Oh, and Credence also serves regular files, duh

### Networking

* HTTP/2 and HTTP/1.1 (HTTP/3 is in the works; no rush)
* Multiple sites can share the same port as long as they are attached to different host names
* Each host name gets its own TLS keys—even when multiple hosts share the same port!

Under the Hood
--------------

* Profoundly asynchronous (scales up gracefully), covering all networking, file access, caching, and compression algorithms
* Automagical in-memory caching (using [kutil_http](https://docs.rs/kutil-http/latest/kutil_http/tower/caching/struct.CachingLayer.html))
* Automagical response compression (integrated into the cache)
* [Rust](https://www.rust-lang.org/), [Tokio](https://github.com/tokio-rs/tokio), [Hyper](https://github.com/hyperium/hyper), [axum](https://github.com/tokio-rs/axum), [Tower](https://github.com/tower-rs/tower) (a.k.a. the "RTHaT stack"?)

License
-------

Like much of the Rust ecosystem, licensed under your choice of either of

* [Apache License, Version 2.0](https://github.com/tliron/rust-credence/blob/main/LICENSE-APACHE)
* [MIT license](https://github.com/tliron/rust-credence/blob/main/LICENSE-MIT)

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
