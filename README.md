*Work in progress*

[![crates.io](https://img.shields.io/crates/v/credence?color=%23227700)](https://crates.io/crates/credence)

Credence
========

An opinionated web server designed for straightforward authoring and scalable performance.

Suitable for blogs, newsletters, personal sites, documentation, etc.

There is no integrated content editor. The premise is that you write your content in simple text
files (with Markdown markup) and let Credence do the rest.

If you like databases, frameworks, "stacks", buzzwords, and bloat then Credence is probably not for
you. This is a no-nonsense web zone.

Usability Features:

* Distributed as a single executable that you just need to point at your content directory
* Unfussy configuration in YAML
* Sensible defaults

Authoring Features:

* Your directory structure = your URL structure (with some streamlining)
* Write your content in Markdown files, with a few optional annotations
* Design your templates in boring old HTML (with optional Jinja conveniences)
* Turn a directory full of Markdown files into a searchable catalog, both as HTML and
  as a JSON endpoint (for your fancy JavaScript widgets)
* Oh, and Credence also serves regular files, duh

Technology:

* Asynchronous request handling (scales up gracefully)
* Automatic in-memory caching
* Automatic response compression (integrated into the cache)
* Stack: [Rust](https://www.rust-lang.org/), [Tokio](https://github.com/tokio-rs/tokio),
  [Hyper](https://github.com/hyperium/hyper), [axum](https://github.com/tokio-rs/axum),
  [Tower](https://github.com/tower-rs/tower) (a.k.a. "RTHaT"...?)

License
-------

Like much of the Rust ecosystem, licensed under your choice of either of

* [Apache License, Version 2.0](LICENSE-APACHE)
* [MIT license](LICENSE-MIT)

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
