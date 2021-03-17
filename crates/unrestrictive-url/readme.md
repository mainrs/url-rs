# unrestrictive-url

> A lightweight wrapper around [url](https://crates.io/crates/url) to allow for free URL modification.

The `url` crate strictly follows the [WHATWG](https://url.spec.whatwg.org/) standard which means that some operations (like changing the protocol from `https` to `whatever`) are strictly forbidden.

This crate is a lightweight wrapper around the `url` crate. It uses `url` to parse a URL but allows for free modification afterwards. `UnrestrictiveUrl`s implement `std::fmt::Display`.
