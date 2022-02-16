[![Tests](https://github.com/kkostov/telemetrydeck-wasm/actions/workflows/tests.yml/badge.svg)](https://github.com/kkostov/telemetrydeck-wasm/actions/workflows/tests.yml)
[![Lint & Format](https://github.com/kkostov/telemetrydeck-wasm/actions/workflows/lint.yml/badge.svg)](https://github.com/kkostov/telemetrydeck-wasm/actions/workflows/lint.yml)
[![Crate](https://img.shields.io/crates/v/telemetrydeck-wasm.svg)](https://crates.io/crates/telemetrydeck-wasm)
[![API](https://docs.rs/telemetrydeck-wasm/badge.svg)](https://docs.rs/telemetrydeck-wasm)

(unofficial) [TelemetryDeck](https://telemetrydeck.com) client for fast and reliable libraries and apps using Rust and WebAssembly

The library provides a wrapper around the TelemetryDeck endpoint for broadcasting signals.

```rust

let client = TelemetryDeck::new("XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX");
client.send("addOne", Some("user"), None, None);
```

Check the example folder for a working solution using Yew.
