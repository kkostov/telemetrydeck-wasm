use clap::Parser;
use std::collections::HashMap;
use telemetrydeck_wasm::TelemetryDeck;

#[derive(Parser)]
#[command(name = "telemetrydeck-cli")]
#[command(about = "Send telemetry signals to TelemetryDeck", long_about = None)]
struct Cli {
    /// TelemetryDeck App ID
    #[arg(short, long)]
    app_id: String,

    /// Signal type to send
    #[arg(short, long)]
    signal: String,

    /// Optional user identifier (will be hashed)
    #[arg(short, long)]
    user: Option<String>,

    /// Optional namespace for multi-tenant deployments
    #[arg(short, long)]
    namespace: Option<String>,

    /// Optional salt to append to user identifier before hashing (recommended: 64 random chars)
    #[arg(long)]
    salt: Option<String>,

    /// Optional floating-point value to include with the signal
    #[arg(long)]
    float_value: Option<f64>,

    /// Use send_sync() instead of send() to get error feedback
    #[arg(long)]
    use_sync: bool,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let client = if cli.namespace.is_some() || cli.salt.is_some() {
        TelemetryDeck::new_with_config(&cli.app_id, cli.namespace, cli.salt, HashMap::new())
    } else {
        TelemetryDeck::new(&cli.app_id)
    };

    if cli.use_sync {
        // Using send_sync() - returns Result for error handling
        println!("Sending signal '{}' (synchronous mode)...", cli.signal);
        match client
            .send_sync(
                &cli.signal,
                cli.user.as_deref(),
                None,
                Some(false), // Not a test signal
                cli.float_value,
            )
            .await
        {
            Ok(()) => println!("✓ Signal sent successfully!"),
            Err(e) => eprintln!("✗ Failed to send signal: {}", e),
        }
    } else {
        // Using send() - fire-and-forget, never returns errors
        println!("Sending signal '{}' (fire-and-forget mode)...", cli.signal);
        client.send(&cli.signal, cli.user.as_deref(), None, Some(false), cli.float_value);
        println!("✓ Signal dispatched (fire-and-forget)");

        // Give the background task time to complete before the CLI exits
        // In a long-running application, the tokio runtime would stay alive
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}
