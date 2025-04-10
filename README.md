*Work in progress*

[![crates.io](https://img.shields.io/crates/v/credence?color=%23227700)](https://crates.io/crates/credence)

Credence
========

An unfussy web server designed for straightforward authoring and scalable performance.

Suitable for hosting blogs, newsletters, personal sites, documentation, etc.

Do you like databases, frameworks, "stacks", buzzwords, and bloat? Do you know what a "CMS" is? Then Credence is probably not for you. Welcome to the nonsense-free web zone.

Proudly missing:

* An integrated content editor. Credence relies on [Markdown](https://en.wikipedia.org/wiki/Markdown) for authoring, so all you need is a text editor. (We have some recommendations.)
* Change tracking. Apparently, git is pretty good at this. (And it can be used 100% locally. You do *not* need a repository hosting service.)

Hey! Developers! Pretty much all of Credence's functionality lives in [credence-lib](https://crates.io/crates/credence), so you can remix Credence with all the bells and whistles your precious little heart desires. You probably want to turn it into a CMS. Sigh.

Documentation
-------------

* Follow the [guide](GUIDE.md) to learn how to use it.
* Refer to the [configuration reference](CONFIGURATION.md).
* We also have some useful [tips](TIPS.md)!

Features
--------

### Usability

* Distributed as a single executable that you just need to point at your web assets directory
* Straightforward configuration in YAML (it's optional)
* Sensible out-of-the-box defaults

### Authoring

* Your directory structure = your URL structure (with some quirks)
* Write your content in Markdown files (with a few optional annotations)
* Design your HTML in ... HTML! (with optional [Jinja](https://en.wikipedia.org/wiki/Jinja_\(template_engine\)) conveniences)
* Turn a directory full of Markdown pages into a user-searchable catalog, both as HTML and
  as a JSON endpoint (for your fancy JavaScript widgets)
* Oh, and Credence also serves regular files, duh

Technology
----------

* Asynchronous request handling (scales up gracefully)
* Automagical in-memory caching (using
  [kutil_http](https://docs.rs/kutil-http/latest/kutil_http/tower/caching/struct.CachingLayer.html))
* Automagical response compression (integrated into the cache)
* Under the hood: [Rust](https://www.rust-lang.org/), [Tokio](https://github.com/tokio-rs/tokio),
  [Hyper](https://github.com/hyperium/hyper), [axum](https://github.com/tokio-rs/axum),
  [Tower](https://github.com/tower-rs/tower) (a.k.a. the "RTHaT stack"?)

License
-------

Like much of the Rust ecosystem, licensed under your choice of either of

* [Apache License, Version 2.0](LICENSE-APACHE)
* [MIT license](LICENSE-MIT)

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
