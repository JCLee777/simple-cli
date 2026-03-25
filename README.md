# simple-cli

A minimal Rust CLI example distributed with cargo-dist.

## Usage

```bash
cargo run -- hello --name Rust
cargo run -- sum 12 30
```

## Build a release binary

```bash
cargo build --release
```

## Generate distribution artifacts with cargo-dist

```bash
dist generate
dist build --artifacts=local
```

## Notes

- `dist-workspace.toml` contains the generated cargo-dist configuration.
