# TelemetryDeck CLI Example

This example demonstrates how to use the TelemetryDeck client in a native Rust CLI application.

## Usage

The CLI supports two modes of sending signals:

### Fire-and-Forget Mode (default)

Uses `send()` which dispatches signals asynchronously without waiting for or returning errors:

```bash
cargo run --manifest-path examples/cli/Cargo.toml -- \
  --app-id "YOUR-APP-ID" \
  --signal "mySignalType" \
  --user "user123"
```

### Synchronous Mode (with error handling)

Uses `send_sync()` which returns a `Result` so you can handle errors:

```bash
cargo run --manifest-path examples/cli/Cargo.toml -- \
  --app-id "YOUR-APP-ID" \
  --signal "mySignalType" \
  --user "user123" \
  --use-sync
```

### Multi-tenant Deployment (with namespace)

For multi-tenant deployments, specify a namespace:

```bash
cargo run --manifest-path examples/cli/Cargo.toml -- \
  --app-id "YOUR-APP-ID" \
  --signal "mySignalType" \
  --user "user123" \
  --namespace "my-tenant"
```

### Enhanced Privacy (with salt)

For additional user identifier privacy, provide a salt:

```bash
cargo run --manifest-path examples/cli/Cargo.toml -- \
  --app-id "YOUR-APP-ID" \
  --signal "mySignalType" \
  --user "user123" \
  --salt "your-64-char-random-salt-here"
```

### With Floating-Point Value

Include a floating-point value with your signal:

```bash
cargo run --manifest-path examples/cli/Cargo.toml -- \
  --app-id "YOUR-APP-ID" \
  --signal "revenue" \
  --user "user123" \
  --float-value 99.99
```

## Command-Line Options

- `--app-id <APP_ID>` (required): Your TelemetryDeck application ID
- `--signal <SIGNAL>` (required): The signal type to send
- `--user <USER>` (optional): User identifier (will be SHA-256 hashed automatically)
- `--namespace <NAMESPACE>` (optional): Namespace for multi-tenant deployments (uses v2 API)
- `--salt <SALT>` (optional): Salt to append to user identifier before hashing (recommended: 64 random chars)
- `--float-value <VALUE>` (optional): Floating-point value to include with the signal
- `--use-sync` (optional): Use synchronous mode with error handling

## When to Use Each Mode

**Fire-and-Forget (`send()`)**:
- Best for telemetry where you don't need to know if it succeeded
- Lower overhead, non-blocking
- Typical for analytics and metrics

**Synchronous (`send_sync()`)**:
- When you need to handle errors (e.g., invalid credentials, network issues)
- For critical telemetry where confirmation is needed
- During development/debugging

## Requirements

- Rust 2021 edition or later
- tokio runtime (included in dependencies)
- Valid TelemetryDeck app ID

## Building

```bash
cargo build --manifest-path examples/cli/Cargo.toml
```

## Running from the project root

```bash
cd examples/cli
cargo run -- --app-id "YOUR-APP-ID" --signal "test"
```
