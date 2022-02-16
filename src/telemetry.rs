use chrono::{DateTime, Utc};
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const CLIENT_VERSION_KEY: &str = "telemetryClientVersion";

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]

/// An instance of an outgoing telemetry signal
pub struct Signal {
    /// When was this signal generated
    pub received_at: DateTime<Utc>,
    /// The App ID of this signal
    #[serde(rename = "appID")]
    pub app_id: String,
    /// A user identifier. This should be hashed on the client, and will be hashed + salted again on the server to break any connection to personally identifiable data.
    pub client_user: String,
    /// A randomly generated session identifier. Should be the same over the course of the session
    #[serde(rename = "sessionID")]
    pub session_id: String,
    /// A type name for this signal that describes the event that triggered the signal
    #[serde(rename = "type")]
    pub signal_type: String,
    /// Tags in the form "key:value" to attach to the signal
    pub payload: Vec<String>,
    /// If `true`, mark the signal as a testing signal and only show it in a dedicated test mode UI
    pub is_test_mode: String,
}

/// The TelemetryDeck api client
#[derive(Debug)]
pub struct TelemetryDeck {
    /// The base url of the service
    pub url: String,
    /// The App ID for outgoing signals
    pub app_id: String,
    /// Default parameters to be appended on all outgoing signals
    pub default_params: HashMap<String, String>,
    /// The current session identifier
    pub session_id: String,
}

impl TelemetryDeck {
    /// Create a new instance with the specified application id
    pub fn new(app_id: &str) -> Self {
        Self::new_with_params(app_id, HashMap::new())
    }

    /// Create a new instance with the specified application id and default parameters
    pub fn new_with_params(app_id: &str, params: HashMap<String, String>) -> Self {
        TelemetryDeck {
            url: String::from("https://nom.telemetrydeck.com"),
            app_id: app_id.to_string(),
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

    /// Send a telemetry signal
    pub fn send(
        &self,
        signal_type: &str,
        client_user: Option<&str>,
        payload: Option<HashMap<String, String>>,
        is_test_mode: Option<bool>,
    ) {
        let signal = self.create_signal(signal_type, client_user, payload, is_test_mode);
        self.send_one(signal);
    }

    fn create_signal(
        &self,
        signal_type: &str,
        client_user: Option<&str>,
        payload: Option<HashMap<String, String>>,
        is_test_mode: Option<bool>,
    ) -> Signal {
        let params = Self::adding_params(&self.default_params, payload);
        let payload = Self::encoded_payload(params);

        let client_user = client_user.map_or_else(
            || "rust".to_string(),
            |u| {
                let mut sha256 = Sha256::new();
                sha256.update(u);
                format!("{:X}", sha256.finalize()).to_lowercase()
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
        }
    }

    fn send_one(&self, signal: Signal) {
        self.send_many(vec![signal])
    }

    fn send_many(&self, signals: Vec<Signal>) {
        let app_id = self.app_id.clone();
        let base_url = &self.url;
        let url = format!("{base_url}/api/v1/apps/{app_id}/signals/multiple/");
        spawn_local(async move {
            let body = serde_json::to_string(&signals).unwrap();
            let resp = Request::post(&url)
                .body(body)
                .header("Content-Type", "application/json")
                .send()
                .await
                .unwrap();
            println!("response {:?}", resp);
        });
    }

    fn adding_params(
        params1: &HashMap<String, String>,
        params2: Option<HashMap<String, String>>,
    ) -> HashMap<String, String> {
        match params2 {
            Some(params) => params1
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .chain(params)
                .collect(),
            None => params1
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
        }
    }

    fn encoded_payload(params: HashMap<String, String>) -> Vec<String> {
        params
            .into_iter()
            .map(|(k, v)| format!("{}:{}", k.replace(":", "_"), v))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::TelemetryDeck;
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    #[test]
    fn create_signal_without_user() {
        let sut = TelemetryDeck::new("1234");
        let result = sut.create_signal("signal_type", None, None, None);
        assert_eq!(result.client_user, "rust".to_string());
        assert_eq!(result.signal_type, "signal_type".to_string());
        assert_eq!(result.app_id, "1234".to_string());
        assert_eq!(result.is_test_mode, "false".to_string());
        assert_eq!(
            result.payload,
            vec![format!("telemetryClientVersion:{VERSION}")]
        );
    }

    #[test]
    fn create_signal_with_user_is_hashed() {
        let sut = TelemetryDeck::new("1234");
        let result = sut.create_signal("signal_type", Some("clientUser"), None, None);
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
