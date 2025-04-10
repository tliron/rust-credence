*Work in progress*

[![crates.io](https://img.shields.io/crates/v/credence?color=%23227700)](https://crates.io/crates/credence)

Credence
========

An unfussy web server designed for straightforward authoring and scalable performance.

Suitable for hosting blogs, newsletters, personal sites, documentation, etc.

Do you like databases, frameworks, "stacks", buzzwords, and bloat? Do you know what a "CMS" is? Then Credence is probably not for you. Welcome to the nonsense-free web zone.

Rustaceans: Pretty much all of Credence's functionality lives in [credence-lib](https://crates.io/crates/credence), so you can enhance it, integrate it into something else, remix it, etc.

Documentation
-------------

* The [basics guide](documentation/basics.md) will get you up and running with the default configuration.
* The [advanced guide](documentation/advanced.md) will take it to the next level.
* We also have some useful [tips](documentation/tips.md)!
* And for rustaceans then there's the [credence-lib documentation](https://docs.rs/credence-lib/latest/credence_lib/).

Features
--------

### Usability

* Distributed as a single executable that you just need to point at your web assets directories
* Serve multiple sites from a single Credence process
* Sensible out-of-the-box defaults
* Straightforward configuration in YAML if you need to go beyond the defaults

### Authoring

* Your directory structure = your URL structure (with some quirks)
* Write your content in [Markdown](https://en.wikipedia.org/wiki/Markdown) (with a few optional annotations)
* Design your HTML in ... HTML! (with optional [Jinja](https://en.wikipedia.org/wiki/Jinja_\(template_engine\)) conveniences)
* Turn a directory full of Markdown pages into a user-searchable catalog, both as HTML and as a JSON endpoint (for your fancy JavaScript widgets)
* Oh, and Credence also serves regular files, duh

### Networking

* HTTP/2 and HTTP/1.1 (HTTP/3 is in the works; no rush)
* Multiple sites can share the same port (as long as they have different host names)
* Each host name can have its own TLS keys (even if they are on the same port! no need for a reverse proxy!)
* Built-in [ACME](https://en.wikipedia.org/wiki/Automatic_Certificate_Management_Environment) client that can auto-renew your TLS keys (no need for [Certbot](https://certbot.eff.org/)!)

Under the Hood
--------------

* Profoundly asynchronous (scales up gracefully)
* Automagical in-memory caching (using [kutil_http](https://docs.rs/kutil-http/latest/kutil_http/tower/caching/struct.CachingLayer.html))
* Automagical response compression (integrated into the cache)
* [Rust](https://www.rust-lang.org/), [Tokio](https://github.com/tokio-rs/tokio), [Hyper](https://github.com/hyperium/hyper), [axum](https://github.com/tokio-rs/axum), [Tower](https://github.com/tower-rs/tower) (a.k.a. the "RTHaT stack"?)

License
-------

Like much of the Rust ecosystem, licensed under your choice of either of

* [Apache License, Version 2.0](LICENSE-APACHE)
* [MIT license](LICENSE-MIT)

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
