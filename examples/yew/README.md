# TelemetryDeck Yew Example

This example demonstrates how to use the TelemetryDeck client in a WebAssembly application built with [Yew](https://yew.rs/).

## What it does

A simple counter application that sends a telemetry signal to TelemetryDeck each time you click the "+1" button.

## Prerequisites

Install [trunk](https://trunkrs.dev/):

```bash
cargo install trunk
```

## Setup

1. Replace `XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX` in `src/main.rs` (line 22) with your TelemetryDeck application ID
2. Run the development server:

```bash
trunk serve
```

3. Open your browser to `http://localhost:8080`
4. Click the "+1" button to send telemetry signals

## Building for production

```bash
trunk build --release
```

The output will be in the `dist/` directory.

## How it works

The example creates a `TelemetryDeck` client instance when the Yew component is initialized. Each button click triggers the `AddOne` message, which sends a signal using `client.send()` before incrementing the counter.

Since this is a fire-and-forget operation, the UI remains responsive and doesn't wait for the network request to complete.

## More Information

See the [Yew documentation](https://yew.rs/docs/getting-started/project-setup/using-trunk) for more details on using trunk with Yew.
