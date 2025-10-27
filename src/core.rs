use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use uuid::Uuid;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const CLIENT_VERSION_KEY: &str = "telemetryClientVersion";

/// An instance of an outgoing telemetry signal
///
///This struct represents a single telemetry event that will be sent to TelemetryDeck.
/// Signals are automatically created by [`TelemetryDeck::send`] and [`TelemetryDeck::send_sync`]
/// methods - you typically don't need to construct these manually.
///
/// # Serialization
///
/// This struct serializes to JSON in camelCase format as expected by the TelemetryDeck API:
///
/// ```json
/// {
///   "receivedAt": "2025-01-15T10:30:00Z",
///   "appID": "xxx-xxx-xxx",
///   "clientUser": "hashed-user-id",
///   "sessionID": "session-uuid",
///   "type": "signalType",
///   "payload": ["key1:value1", "key2:value2"],
///   "isTestMode": "false",
///   "floatValue": 42.5
/// }
/// ```
///
/// # Privacy
///
/// - `client_user` is always SHA-256 hashed before being set
/// - `session_id` is a UUID v4 generated per client instance
/// - `is_test_mode` is serialized as a string ("true" or "false")
/// - `float_value` is omitted from JSON when `None`
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Signal {
    /// Timestamp when this signal was generated (UTC)
    pub received_at: DateTime<Utc>,

    /// The TelemetryDeck App ID this signal belongs to
    #[serde(rename = "appID")]
    pub app_id: String,

    /// SHA-256 hashed user identifier
    ///
    /// This value is automatically hashed by the client before transmission.
    /// The server will hash it again with its own salt for privacy.
    pub client_user: String,

    /// Session identifier (UUID v4)
    ///
    /// Persists for the lifetime of a [`TelemetryDeck`] instance.
    /// Can be reset using [`TelemetryDeck::reset_session`].
    #[serde(rename = "sessionID")]
    pub session_id: String,

    /// The type/name of this signal (e.g., "userLogin", "buttonClick")
    #[serde(rename = "type")]
    pub signal_type: String,

    /// Custom parameters encoded as "key:value" strings
    ///
    /// Created from the HashMap passed to `send()` or `send_sync()`.
    /// Keys containing colons are sanitized (`:` → `_`).
    pub payload: Vec<String>,

    /// Whether this is a test signal (serialized as string "true" or "false")
    ///
    /// Test signals are shown separately in the TelemetryDeck UI.
    pub is_test_mode: String,

    /// Optional floating-point value associated with this signal
    ///
    /// Useful for tracking numeric metrics like revenue, duration, score, etc.
    /// Omitted from JSON when `None`.
    #[serde(rename = "floatValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub float_value: Option<f64>,
}

/// TelemetryDeck API client
///
/// This is the main entry point for sending telemetry signals to TelemetryDeck.
/// The client handles session management, user identifier hashing, and HTTP communication.
///
/// # Examples
///
/// ## Basic Usage
///
/// ```no_run
/// use telemetrydeck_wasm::TelemetryDeck;
///
/// let client = TelemetryDeck::new("YOUR-APP-ID");
/// client.send("userLogin", Some("user@example.com"), None, None, None);
/// ```
///
/// ## With Configuration
///
/// ```no_run
/// use telemetrydeck_wasm::TelemetryDeck;
/// use std::collections::HashMap;
///
/// let client = TelemetryDeck::new_with_config(
///     "YOUR-APP-ID",
///     Some("my-tenant".to_string()),  // namespace
///     Some("random-salt-64-chars".to_string()),  // salt
///     HashMap::new(),  // default params
/// );
/// ```
///
/// # Platform Support
///
/// - **Native**: Uses `reqwest` + `tokio::spawn`
/// - **WASM**: Uses `reqwasm` + `spawn_local`
///
/// # Privacy
///
/// - User identifiers are always SHA-256 hashed
/// - Optional salt is concatenated after user ID before hashing
/// - Session IDs are random UUIDs
#[derive(Debug)]
pub struct TelemetryDeck {
    /// Base URL of the TelemetryDeck service
    ///
    /// Default: `https://nom.telemetrydeck.com`
    url: String,

    /// Your TelemetryDeck App ID
    pub app_id: String,

    /// Optional namespace for multi-tenant deployments
    ///
    /// When set, signals are sent to `/v2/namespace/{namespace}/`
    /// instead of `/v2/`.
    pub namespace: Option<String>,

    /// Optional salt for user identifier hashing
    ///
    /// The salt is concatenated after the user identifier before
    /// SHA-256 hashing: `hash(user_id + salt)`.
    ///
    /// # Security Note
    ///
    /// It is recommended to use a cryptographically random salt of at least
    /// 64 characters. The salt should be unique per application but consistent
    /// across all users of the same application.
    pub salt: Option<String>,

    /// Default parameters appended to all outgoing signals
    ///
    /// These are merged with per-signal parameters.
    /// The library version is automatically added as `telemetryClientVersion`.
    pub default_params: HashMap<String, String>,

    /// Current session identifier (UUID v4)
    ///
    /// Generated automatically when the client is created.
    /// Can be reset using [`TelemetryDeck::reset_session`].
    pub session_id: String,
}

impl TelemetryDeck {
    /// Create a new instance with the specified application id
    #[must_use]
    pub fn new(app_id: &str) -> Self {
        Self::new_with_config(app_id, None, None, HashMap::new())
    }

    /// Create a new instance with the specified application id and configuration
    ///
    /// # Examples
    ///
    /// ```
    /// use telemetrydeck_wasm::TelemetryDeck;
    /// use std::collections::HashMap;
    ///
    /// // Create client with namespace for multi-tenant deployment
    /// let client = TelemetryDeck::new_with_config(
    ///     "XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX",
    ///     Some("my-namespace".to_string()),
    ///     None,
    ///     HashMap::new(),
    /// );
    ///
    /// // Create client with salt for enhanced user hashing
    /// let client = TelemetryDeck::new_with_config(
    ///     "XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX",
    ///     None,
    ///     Some("your-64-char-random-salt-here".to_string()),
    ///     HashMap::new(),
    /// );
    ///
    /// // Create client with default parameters
    /// let mut defaults = HashMap::new();
    /// defaults.insert("environment".to_string(), "production".to_string());
    /// let client = TelemetryDeck::new_with_config(
    ///     "XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX",
    ///     None,
    ///     None,
    ///     defaults,
    /// );
    /// ```
    #[must_use]
    pub fn new_with_config(
        app_id: &str,
        namespace: Option<String>,
        salt: Option<String>,
        params: HashMap<String, String>,
    ) -> Self {
        TelemetryDeck {
            url: String::from("https://nom.telemetrydeck.com"),
            app_id: app_id.to_string(),
            namespace,
            salt,
            default_params: Self::adding_params(
                &params,
                Some(HashMap::from([(
                    CLIENT_VERSION_KEY.to_string(),
                    VERSION.to_string(),
                )])),
            ),
            session_id: Uuid::new_v4().to_string(),
        }
    }

    /// Reset the session id for future signals
    pub fn reset_session(&mut self, new_session_id: Option<String>) {
        self.session_id = new_session_id.unwrap_or_else(|| Uuid::new_v4().to_string());
    }

    /// Create a signal with the specified parameters
    pub(crate) fn create_signal(
        &self,
        signal_type: &str,
        client_user: Option<&str>,
        payload: Option<HashMap<String, String>>,
        is_test_mode: Option<bool>,
        float_value: Option<f64>,
    ) -> Signal {
        let params = Self::adding_params(&self.default_params, payload);
        let payload = Self::encoded_payload(params);

        let client_user = client_user.map_or_else(
            || "rust".to_string(),
            |u| {
                let user_with_salt = if let Some(salt) = &self.salt {
                    format!("{}{}", u, salt)
                } else {
                    u.to_string()
                };
                let mut sha256 = Sha256::new();
                sha256.update(user_with_salt.as_bytes());
                format!("{:x}", sha256.finalize())
            },
        );
        Signal {
            received_at: Utc::now(),
            app_id: self.app_id.clone(),
            client_user,
            session_id: self.session_id.clone(),
            signal_type: signal_type.to_string(),
            payload,
            is_test_mode: is_test_mode.unwrap_or(false).to_string(),
            float_value,
        }
    }

    /// Build the API URL for sending signals
    pub(crate) fn build_url(&self) -> String {
        if let Some(namespace) = &self.namespace {
            format!("{}/v2/namespace/{}/", self.url, namespace)
        } else {
            format!("{}/v2/", self.url)
        }
    }

    fn adding_params(
        params1: &HashMap<String, String>,
        params2: Option<HashMap<String, String>>,
    ) -> HashMap<String, String> {
        let mut result = params1.clone();
        if let Some(params) = params2 {
            result.extend(params);
        }
        result
    }

    /// Encode parameters as "key:value" strings
    ///
    /// Colons in parameter keys are replaced with underscores to avoid
    /// conflicts with the "key:value" encoding format.
    fn encoded_payload(params: HashMap<String, String>) -> Vec<String> {
        params
            .into_iter()
            .map(|(k, v)| format!("{}:{}", k.replace(':', "_"), v))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::TelemetryDeck;
    use std::collections::HashMap;
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    #[test]
    fn create_signal_without_user() {
        let sut = TelemetryDeck::new("1234");
        let result = sut.create_signal("signal_type", None, None, None, None);
        assert_eq!(result.client_user, "rust".to_string());
        assert_eq!(result.signal_type, "signal_type".to_string());
        assert_eq!(result.app_id, "1234".to_string());
        assert_eq!(result.is_test_mode, "false".to_string());
        assert_eq!(
            result.payload,
            vec![format!("telemetryClientVersion:{VERSION}")]
        );
        assert_eq!(result.float_value, None);
    }

    #[test]
    fn create_signal_with_user_is_hashed() {
        let sut = TelemetryDeck::new("1234");
        let result = sut.create_signal("signal_type", Some("clientUser"), None, None, None);
        assert_eq!(
            result.client_user,
            "6721870580401922549fe8fdb09a064dba5b8792fa018d3bd9ffa90fe37a0149".to_string()
        );
        assert_eq!(result.signal_type, "signal_type".to_string());
        assert_eq!(result.app_id, "1234".to_string());
        assert_eq!(result.is_test_mode, "false".to_string());
        assert_eq!(
            result.payload,
            vec![format!("telemetryClientVersion:{VERSION}")]
        );
    }

    #[test]
    fn create_signal_with_user_and_salt_is_hashed() {
        let sut = TelemetryDeck::new_with_config(
            "1234",
            None,
            Some("someSalt".to_string()),
            HashMap::new(),
        );
        let result = sut.create_signal("signal_type", Some("clientUser"), None, None, None);
        assert_eq!(
            result.client_user,
            "ffdd613ce521b2e94b8931bdadffd96857f6abbde6c0ee1fcf0b76127fbb9e5a".to_string()
        );
    }

    #[test]
    fn create_signal_with_float_value() {
        let sut = TelemetryDeck::new("1234");
        let result = sut.create_signal("signal_type", None, None, None, Some(42.5));
        assert_eq!(result.float_value, Some(42.5));

        // Verify serialization includes floatValue
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"floatValue\":42.5"));
    }

    #[test]
    fn create_signal_without_float_value_omits_field() {
        let sut = TelemetryDeck::new("1234");
        let result = sut.create_signal("signal_type", None, None, None, None);
        assert_eq!(result.float_value, None);

        // Verify serialization omits floatValue when None
        let json = serde_json::to_string(&result).unwrap();
        assert!(!json.contains("floatValue"));
    }

    #[test]
    fn build_url_without_namespace() {
        let sut = TelemetryDeck::new("1234");
        assert_eq!(sut.build_url(), "https://nom.telemetrydeck.com/v2/");
    }

    #[test]
    fn build_url_with_namespace() {
        let sut = TelemetryDeck::new_with_config(
            "1234",
            Some("my-namespace".to_string()),
            None,
            HashMap::new(),
        );
        assert_eq!(
            sut.build_url(),
            "https://nom.telemetrydeck.com/v2/namespace/my-namespace/"
        );
    }

    #[test]
    fn reset_session() {
        let mut sut = TelemetryDeck::new("1234");
        let session1 = sut.session_id.clone();
        sut.reset_session(None);
        let session2 = sut.session_id.clone();
        assert_ne!(session1, session2);
    }

    #[test]
    fn reset_session_with_specific_id() {
        let mut sut = TelemetryDeck::new("1234");
        sut.reset_session(Some("my session".to_string()));
        let session2 = sut.session_id.clone();
        assert_eq!(session2, "my session".to_string());
    }
}
