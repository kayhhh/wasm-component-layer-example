# wasm-component-layer-example

## Usage

First build the guest WASM using cargo-component.

```bash
cargo component build -p guest --target wasm32-unknown-unknown
```

Then run the native build with:

```bash
cargo run -p host
```

Or the web build with:

```bash
trunk serve
```
