(unofficial) (TelemetryDeck)[https://telemetrydeck.com] client for fast and reliable libraries and apps using Rust and WebAssembly

The library provides a wrapper around the TelemetryDeck endpoint for broadcasting signals.

```rust

let client = TelemetryDeck::new("XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX");
client.send("addOne", Some("user"), None, None);
```

Check the example folder for a working solution using Yew.
