# swaynag &emsp; [![Action Badge]][actions] [![Version Badge]][crates.io] [![License Badge]][license]

[Version Badge]: https://img.shields.io/crates/v/swaynag.svg
[crates.io]: https://crates.io/crates/swaynag
[Action Badge]: https://github.com/JayceFayne/swaynag/workflows/Rust/badge.svg
[actions]: https://github.com/JayceFayne/swaynag/actions
[License Badge]: https://img.shields.io/crates/l/swaynag.svg
[license]: https://github.com/JayceFayne/swaynag/blob/master/LICENSE.md

A simple wrapper around [swaynag](https://github.com/swaywm/sway/blob/master/swaynag/swaynag.1.scd).
This library can be used  in either a synchronous or asynchronous manner.
The async feature can be enabled be adding the following to your Cargo.toml:

```toml
[dependencies.swaynag]
features = ["async"]
```

## Usage

Examples of how to use the library can be found [here](src/test.rs).

## Versioning

This library targets the latest stable release of [sway](https://github.com/swaywm/sway).

## Contributing

 If you find any errors in swaynag or just want to add a new feature feel free to [submit a PR](https://github.com/jaycefayne/swaynag/pulls).
