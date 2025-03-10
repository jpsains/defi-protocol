#Simple defi protocol using Arbitrum Stylus programs in Rust

## Prerequisites

![alt text](<Google Chrome 2025-03-04 12.05.51.png>)

## Quick Start

Install [Rust]

```bash
cargo install --force cargo-stylus cargo-stylus-check
```

Add the `wasm32-unknown-unknown` build target to your Rust compiler:

```
rustup target add wasm32-unknown-unknown
```

You should now have it available as a Cargo subcommand:

```bash
cargo stylus --help
```

Then, run test

```
cargo test --package stylus-hello-world --lib -- test --show-output
```
