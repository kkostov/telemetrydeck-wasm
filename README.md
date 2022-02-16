[![Tests](https://github.com/kkostov/telemetrydeck-wasm/actions/workflows/tests.yml/badge.svg)](https://github.com/kkostov/telemetrydeck-wasm/actions/workflows/tests.yml)
[![Lint & Format](https://github.com/kkostov/telemetrydeck-wasm/actions/workflows/lint.yml/badge.svg)](https://github.com/kkostov/telemetrydeck-wasm/actions/workflows/lint.yml)
[![Crate](https://img.shields.io/crates/v/telemetrydeck-wasm.svg)](https://crates.io/crates/telemetrydeck-wasm)
[![API](https://docs.rs/telemetrydeck-wasm/badge.svg)](https://docs.rs/telemetrydeck-wasm)

# TelemetryDeck Client

Client for integrating private analytics in fast and reliable libraries and apps using Rust and WebAssembly

The library provides a wrapper around the [TelemetryDeck](https://telemetrydeck.com) endpoint for broadcasting signals.

## Sending a signal

```rust

let client = TelemetryDeck::new("XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX");

// Signal a type and a user identifier
client.send("addOne", Some("user"), None, None);

// Signal with custom payload parameters
client.send(
  "signalType",
  Some("user identifier"),
  Some(HashMap::from([("key".to_string(), "value".to_string())])),
  None,
);
```

## Session identifier

When an instance of `TelemetryDeck` is created, it is assigned a session identifier. This identifier persists for all outgoing signals during the lifetime of the instance.

You can reset the session identifier without recreating the client:

```rust
client.reset_session(None)
```

You can also provide your own session identifier:

```rust
client.reset_session(Some("my session id".to_string()));
```

Check the example folder for a working app using [Yew](https://yew.rs).

## Disclaimer

This repository is not affiliated with [TelemetryDeck](https://telemetrydeck.com).
