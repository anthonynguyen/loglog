# `loglog`

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
