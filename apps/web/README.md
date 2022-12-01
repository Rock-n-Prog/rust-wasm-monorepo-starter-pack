# ACME Web App

Yew web app for ACME

## Setup

### Install dependencies for compiling Rust to WASM

```bash
rustup target add wasm32-unknown-unknown
```

## Available scripts

### Install dependencies

TODO: Document how to install deps. Can't we install them form Cargo.toml? -> `cargo install trunk wasm-bindgen-cli`

TODO: Also add to run `sudo apt install libssl-dev`

### Run

```bash
trunk serve

# To watch
trunk watch
```

### Build

```bash
trunk build --release
```

### Lint

```bash
pnpm lint

# To fix lint
pnpm lint:fix
```
