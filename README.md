[![Tests](https://github.com/kkostov/telemetrydeck-wasm/actions/workflows/tests.yml/badge.svg)](https://github.com/kkostov/telemetrydeck-wasm/actions/workflows/tests.yml)
[![Lint & Format](https://github.com/kkostov/telemetrydeck-wasm/actions/workflows/lint.yml/badge.svg)](https://github.com/kkostov/telemetrydeck-wasm/actions/workflows/lint.yml)
[![Crate](https://img.shields.io/crates/v/telemetrydeck-wasm.svg)](https://crates.io/crates/telemetrydeck-wasm)
[![API](https://docs.rs/telemetrydeck-wasm/badge.svg)](https://docs.rs/telemetrydeck-wasm)

# TelemetryDeck Client

Client for integrating private analytics in fast and reliable libraries and apps using Rust. Supports both native Rust applications and WebAssembly.

The library provides a client for the [TelemetryDeck](https://telemetrydeck.com) endpoint for broadcasting signals.

## Installation

Add to your `Cargo.toml`:

**For native Rust applications (servers, CLI tools):**
```toml
[dependencies]
telemetrydeck-wasm = "0.3"
```

**For WebAssembly applications:**
```toml
[dependencies]
telemetrydeck-wasm = { version = "0.3", features = ["wasm"] }
```

## Sending a signal

### Basic Usage

```rust
use telemetrydeck_wasm::TelemetryDeck;
use std::collections::HashMap;

let client = TelemetryDeck::new("XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX");

// Fire-and-forget signal (spawns async task, never blocks)
client.send("addOne", Some("user"), None, None, None);

// Signal with custom payload parameters
client.send(
  "signalType",
  Some("user identifier"),
  Some(HashMap::from([("key".to_string(), "value".to_string())])),
  None,
  None,
);

// Signal with floating-point value
client.send("revenue", Some("user"), None, None, Some(99.99));

// For error handling, use send_sync (returns Result)
match client.send_sync("signalType", Some("user"), None, None, None).await {
    Ok(()) => println!("Signal sent successfully"),
    Err(e) => eprintln!("Failed to send: {}", e),
}
```

### Multi-tenant Deployments (with namespace)

For multi-tenant deployments, you can specify a namespace:

```rust
use telemetrydeck_wasm::TelemetryDeck;
use std::collections::HashMap;

let client = TelemetryDeck::new_with_config(
    "XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX",
    Some("my-namespace".to_string()),
    None,
    HashMap::new(),
);

client.send("signalType", Some("user"), None, None, None);
```

### Enhanced User Hashing (with salt)

For additional privacy, you can provide a salt that will be concatenated with user identifiers before hashing:

```rust
use telemetrydeck_wasm::TelemetryDeck;
use std::collections::HashMap;

// It's recommended to use a 64-character random salt
let client = TelemetryDeck::new_with_config(
    "XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX",
    None,
    Some("your-64-char-random-salt-here".to_string()),
    HashMap::new(),
);

client.send("signalType", Some("user"), None, None, None);
```

### Reserved Signal Types and Parameters

The library provides constants for [reserved signal](https://telemetrydeck.com/docs/ingest/default-parameters/) types and parameters defined by other TelemetryDeck SDKs:

```rust
use telemetrydeck_wasm::TelemetryDeck;
use telemetrydeck_wasm::signals;
use telemetrydeck_wasm::params;
use std::collections::HashMap;

let client = TelemetryDeck::new("XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX");

client.send(signals::session::STARTED, Some("user"), None, None, None);

let payload = HashMap::from([
    (params::device::PLATFORM.to_string(), "web".to_string()),
    (params::device::ARCHITECTURE.to_string(), "wasm32".to_string()),
]);

client.send("customSignal", Some("user"), Some(payload), None, Some(42.5));
```

Available signal constants include:
- `signals::session::STARTED`
- `signals::navigation::PATH_CHANGED`
- `signals::purchase::COMPLETED`
- `signals::acquisition::NEW_INSTALL_DETECTED`

Available parameter constants are organized by category:
- `params::accessibility::*` - Accessibility settings
- `params::device::*` - Device information
- `params::navigation::*` - Navigation data
- `params::purchase::*` - Purchase information
- `params::retention::*` - User retention metrics
- `params::calendar::*` - Time-based data
- `params::run_context::*` - Runtime environment
- `params::user_preferences::*` - User preferences

See the [API documentation](https://docs.rs/telemetrydeck-wasm) for a complete list.

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

## Examples

This repository includes two complete examples:

### WebAssembly Example (Yew)
A simple counter web application built with [Yew](https://yew.rs) that sends telemetry signals on button clicks.

**Location:** `examples/yew/`

**Run:**
```bash
cd examples/yew
trunk serve
```

Requires [trunk](https://trunkrs.dev/) to be installed. See `examples/yew/README.md` for details.

### Native CLI Example
A command-line application demonstrating native Rust usage with both `send()` and `send_sync()` methods.

**Location:** `examples/cli/`

**Run:**
```bash
cargo run --manifest-path examples/cli/Cargo.toml -- \
  --app-id "YOUR-APP-ID" \
  --signal "test" \
  --user "user123"
```

The CLI supports namespace and salt options:
```bash
cargo run --manifest-path examples/cli/Cargo.toml -- \
  --app-id "YOUR-APP-ID" \
  --signal "test" \
  --user "user123" \
  --namespace "my-tenant" \
  --salt "my-64-char-random-salt"
```

See `examples/cli/README.md` for all options and usage details.

## API Changes

### Breaking Changes in 0.4.0

The `send()` and `send_sync()` methods now include an additional parameter for `float_value`:

```rust
// Before (0.3.x)
client.send(signal_type, user, payload, is_test_mode);

// After (0.4.0)
client.send(signal_type, user, payload, is_test_mode, float_value);
```

To migrate, add `None` as the fifth parameter if you don't need to send a float value:

```rust
client.send("signalType", Some("user"), None, None, None);
```

### Migration from 0.2.x

If you're upgrading from version 0.2.x, you need to add the `wasm` feature flag to your `Cargo.toml`:

```toml
# Before (0.2.x)
[dependencies]
telemetrydeck-wasm = "0.2"

# After (0.4.x)
[dependencies]
telemetrydeck-wasm = { version = "0.4", features = ["wasm"] }
```

## Disclaimer

This repository is not affiliated with [TelemetryDeck](https://telemetrydeck.com).
