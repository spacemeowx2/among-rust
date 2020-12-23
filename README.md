## Prerequisites

```
cargo install cargo-make
```

### Build and serve WASM version

```
cargo make serve
```

### Native

```
cargo make run
```

## Fast compiles

This project enabled LLD linker. You can disable it by deleting `.cargo/config.toml`

See: [Offical guide](https://bevyengine.org/learn/book/getting-started/setup/#enable-fast-compiles-optional)
