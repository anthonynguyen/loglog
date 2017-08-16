# `loglog`

[![loglog on Travis CI][travis-image]][travis]
[![loglog on crates.io][cratesio-image]][cratesio]
[![loglog on docs.rs][docsrs-image]][docsrs]

[travis-image]: https://travis-ci.org/anthonynguyen/loglog.svg?branch=master
[travis]: https://travis-ci.org/anthonynguyen/loglog
[cratesio-image]: https://img.shields.io/crates/v/loglog.svg
[cratesio]: https://crates.io/crates/loglog
[docsrs-image]: https://docs.rs/loglog/badge.svg
[docsrs]: https://docs.rs/chrono/

A simple and usable logger.

## Usage

Create the builder:

```rust
loglog::build()
```

Configure the options:

```rust
loglog::build()
    .time(Some("%H:%M"))
    .stdout(true)
```

Finally, set up and use the logger:

```rust
loglog::build()
    .time(Some("%H:%M"))
    .stdout(true)
    .init()
    .unwrap();

info!("Hello!");
```

For more information, please see the [documentation](https://docs.rs/loglog/).

## License

loglog is licensed under the MIT license. Please see the `LICENSE` file for more
details.
