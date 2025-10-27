use crate::core::{Signal, TelemetryDeck};
use std::collections::HashMap;

impl TelemetryDeck {
    /// Send a telemetry signal (fire-and-forget)
    ///
    /// This method spawns an async task using `tokio::spawn` and never returns errors.
    /// The signal is sent in the background without blocking. Use [`send_sync`](Self::send_sync)
    /// if you need error handling.
    ///
    /// # Parameters
    ///
    /// * `signal_type` - The type/name of the signal (e.g., "userLogin", "buttonClick")
    /// * `client_user` - Optional user identifier. Will be SHA-256 hashed automatically.
    ///   If `None`, defaults to "rust".
    /// * `payload` - Optional key-value parameters to attach to the signal
    /// * `is_test_mode` - Whether to mark this as a test signal. Defaults to `false` if `None`.
    /// * `float_value` - Optional floating-point value (useful for metrics like revenue, duration, etc.)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use telemetrydeck_wasm::TelemetryDeck;
    ///
    /// let client = TelemetryDeck::new("YOUR-APP-ID");
    ///
    /// // Simple signal
    /// client.send("buttonClick", None, None, None, None);
    ///
    /// // With user
    /// client.send("userLogin", Some("user@example.com"), None, None, None);
    ///
    /// // With float value for revenue tracking
    /// client.send("purchase", Some("user123"), None, None, Some(49.99));
    /// ```
    ///
    /// ```no_run
    /// use telemetrydeck_wasm::TelemetryDeck;
    /// use std::collections::HashMap;
    ///
    /// let client = TelemetryDeck::new("YOUR-APP-ID");
    ///
    /// let mut params = HashMap::new();
    /// params.insert("screen".to_string(), "settings".to_string());
    /// params.insert("action".to_string(), "toggle".to_string());
    ///
    /// client.send("userAction", Some("user"), Some(params), None, None);
    /// ```
    ///
    /// # Platform Note
    ///
    /// On native platforms, this requires a tokio runtime to be running.
    pub fn send(
        &self,
        signal_type: &str,
        client_user: Option<&str>,
        payload: Option<HashMap<String, String>>,
        is_test_mode: Option<bool>,
        float_value: Option<f64>,
    ) {
        let signal =
            self.create_signal(signal_type, client_user, payload, is_test_mode, float_value);
        self.send_one(signal);
    }

    /// Send a telemetry signal and return errors if any occur
    ///
    /// This method waits for the HTTP request to complete and returns a `Result`.
    /// Use this when you need to know if the signal was sent successfully.
    ///
    /// # Parameters
    ///
    /// * `signal_type` - The type/name of the signal (e.g., "userLogin", "buttonClick")
    /// * `client_user` - Optional user identifier. Will be SHA-256 hashed automatically.
    ///   If `None`, defaults to "rust".
    /// * `payload` - Optional key-value parameters to attach to the signal
    /// * `is_test_mode` - Whether to mark this as a test signal. Defaults to `false` if `None`.
    /// * `float_value` - Optional floating-point value (useful for metrics like revenue, duration, etc.)
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the signal was sent successfully (HTTP 2xx status)
    /// * `Err(...)` if sending failed (network error, HTTP error, serialization error, etc.)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use telemetrydeck_wasm::TelemetryDeck;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = TelemetryDeck::new("YOUR-APP-ID");
    ///
    /// // Handle errors explicitly
    /// match client.send_sync("criticalEvent", Some("user"), None, None, None).await {
    ///     Ok(()) => println!("Signal sent successfully"),
    ///     Err(e) => eprintln!("Failed to send: {}", e),
    /// }
    ///
    /// // Or use ? operator
    /// client.send_sync("anotherEvent", None, None, None, None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_sync(
        &self,
        signal_type: &str,
        client_user: Option<&str>,
        payload: Option<HashMap<String, String>>,
        is_test_mode: Option<bool>,
        float_value: Option<f64>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let signal =
            self.create_signal(signal_type, client_user, payload, is_test_mode, float_value);
        self.send_many_sync(vec![signal]).await
    }

    fn send_one(&self, signal: Signal) {
        self.send_many(vec![signal])
    }

    fn send_many(&self, signals: Vec<Signal>) {
        let url = self.build_url();
        tokio::spawn(async move {
            let client = reqwest::Client::new();
            let body = serde_json::to_string(&signals).unwrap();
            let _resp = client
                .post(&url)
                .body(body)
                .header("Content-Type", "application/json")
                .send()
                .await;
        });
    }

    async fn send_many_sync(&self, signals: Vec<Signal>) -> Result<(), Box<dyn std::error::Error>> {
        let url = self.build_url();
        let client = reqwest::Client::new();
        let body = serde_json::to_string(&signals)?;
        let resp = client
            .post(&url)
            .body(body)
            .header("Content-Type", "application/json")
            .send()
            .await?;

        if resp.status().is_success() {
            Ok(())
        } else {
            Err(format!("HTTP error: {}", resp.status()).into())
        }
    }
}
