//! TelemetryDeck client for Rust applications and WebAssembly
//!
//! This library provides a simple, privacy-focused API for sending telemetry signals to
//! [TelemetryDeck](https://telemetrydeck.com), supporting both native Rust applications
//! and WebAssembly targets.
//!
//! # Features
//!
//! - **Platform Support**: Works in native Rust (using `reqwest` + `tokio`) and WebAssembly (using `reqwasm`)
//! - **Privacy by Default**: Automatic SHA-256 hashing of user identifiers with optional salt
//! - **Multi-tenant Support**: Optional namespace parameter for multi-tenant deployments
//! - **Fire-and-Forget or Error Handling**: Choose between `send()` (async spawn) or `send_sync()` (returns Result)
//! - **Reserved Constants**: Pre-defined signal types and parameter names for common use cases
//! - **Session Management**: Automatic session ID generation and management
//! - **TelemetryDeck v2 API**: Full support for the latest API features
//!
//! # Installation
//!
//! ## Native Rust Applications
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! telemetrydeck-wasm = "0.4"
//! tokio = { version = "1", features = ["rt", "macros"] }
//! ```
//!
//! ## WebAssembly Applications
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! telemetrydeck-wasm = { version = "0.4", features = ["wasm"] }
//! ```
//!
//! # Quick Start
//!
//! ```no_run
//! use telemetrydeck_wasm::TelemetryDeck;
//!
//! // Create a client with your TelemetryDeck App ID
//! let client = TelemetryDeck::new("XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX");
//!
//! // Send a signal (fire-and-forget)
//! client.send("userLogin", Some("user@example.com"), None, None, None);
//! ```
//!
//! # Examples
//!
//! ## Basic Signal
//!
//! ```no_run
//! use telemetrydeck_wasm::TelemetryDeck;
//!
//! let client = TelemetryDeck::new("YOUR-APP-ID");
//! client.send("buttonClick", None, None, None, None);
//! ```
//!
//! ## Signal with User and Custom Parameters
//!
//! ```no_run
//! use telemetrydeck_wasm::TelemetryDeck;
//! use std::collections::HashMap;
//!
//! let client = TelemetryDeck::new("YOUR-APP-ID");
//!
//! let mut params = HashMap::new();
//! params.insert("screen".to_string(), "settings".to_string());
//! params.insert("version".to_string(), "1.2.0".to_string());
//!
//! client.send("appOpened", Some("user123"), Some(params), None, None);
//! ```
//!
//! ## Signal with Floating-Point Value
//!
//! ```no_run
//! use telemetrydeck_wasm::TelemetryDeck;
//!
//! let client = TelemetryDeck::new("YOUR-APP-ID");
//!
//! // Track revenue with a float value
//! client.send("revenue", Some("user123"), None, None, Some(99.99));
//! ```
//!
//! ## Multi-tenant Deployment with Namespace
//!
//! ```no_run
//! use telemetrydeck_wasm::TelemetryDeck;
//! use std::collections::HashMap;
//!
//! let client = TelemetryDeck::new_with_config(
//!     "YOUR-APP-ID",
//!     Some("tenant-xyz".to_string()),
//!     None,
//!     HashMap::new(),
//! );
//!
//! client.send("userAction", Some("user123"), None, None, None);
//! ```
//!
//! ## Enhanced User Privacy with Salt
//!
//! ```no_run
//! use telemetrydeck_wasm::TelemetryDeck;
//! use std::collections::HashMap;
//!
//! // Use a cryptographically random salt (recommended: 64 chars)
//! let client = TelemetryDeck::new_with_config(
//!     "YOUR-APP-ID",
//!     None,
//!     Some("your-64-char-random-salt-here".to_string()),
//!     HashMap::new(),
//! );
//!
//! client.send("userEvent", Some("user@example.com"), None, None, None);
//! ```
//!
//! ## Using Reserved Signal Types and Parameters
//!
//! ```no_run
//! use telemetrydeck_wasm::{TelemetryDeck, signals, params};
//! use std::collections::HashMap;
//!
//! let client = TelemetryDeck::new("YOUR-APP-ID");
//!
//! // Use pre-defined signal types
//! client.send(signals::session::STARTED, None, None, None, None);
//!
//! // Use pre-defined parameter names
//! let mut payload = HashMap::new();
//! payload.insert(params::device::PLATFORM.to_string(), "web".to_string());
//! payload.insert(params::device::ARCHITECTURE.to_string(), "wasm32".to_string());
//!
//! client.send("customSignal", Some("user"), Some(payload), None, None);
//! ```
//!
//! ## Error Handling with send_sync()
//!
//! ```no_run
//! use telemetrydeck_wasm::TelemetryDeck;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = TelemetryDeck::new("YOUR-APP-ID");
//!
//! // Use send_sync() for error handling
//! match client.send_sync("criticalEvent", Some("user"), None, None, None).await {
//!     Ok(()) => println!("Signal sent successfully"),
//!     Err(e) => eprintln!("Failed to send signal: {}", e),
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # Platform-Specific Behavior
//!
//! ## Native Rust (default)
//!
//! - Uses `reqwest` for HTTP requests
//! - Uses `tokio::spawn` for fire-and-forget signals
//! - Requires a tokio runtime to be running
//!
//! ## WebAssembly (with `wasm` feature)
//!
//! - Uses `reqwasm` for HTTP requests
//! - Uses `wasm_bindgen_futures::spawn_local` for fire-and-forget signals
//! - Works in browser event loop (no runtime needed)
//!
//! # Privacy and Security
//!
//! - User identifiers are always SHA-256 hashed before transmission
//! - Optional salt can be added for enhanced privacy
//! - Payload keys with colons are sanitized (`:` → `_`)
//! - Test mode signals can be marked separately
//!
//! # API Version
//!
//! This library uses the TelemetryDeck v2 API:
//! - Default endpoint: `/v2/`
//! - Namespace endpoint: `/v2/namespace/{namespace}/`

#![deny(missing_docs, missing_debug_implementations)]

mod core;
pub use core::{Signal, TelemetryDeck};

/// Reserved signal type constants defined by TelemetryDeck
///
/// See the [signals] module documentation for usage examples.
pub mod signals;

/// Reserved parameter name constants defined by TelemetryDeck
///
/// See the [params] module documentation for usage examples.
pub mod params;

#[cfg(feature = "wasm")]
mod client_wasm;

#[cfg(not(feature = "wasm"))]
mod client_native;
