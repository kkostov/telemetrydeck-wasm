//! Reserved signal type constants defined by TelemetryDeck
//!
//! This module provides pre-defined signal type names for common telemetry events.
//! Using these constants ensures consistency with the TelemetryDeck platform's
//! built-in analytics and dashboards.
//!
//! # Example
//!
//! ```no_run
//! use telemetrydeck_wasm::{TelemetryDeck, signals};
//!
//! let client = TelemetryDeck::new("YOUR-APP-ID");
//!
//! // Send a session started signal
//! client.send(signals::session::STARTED, None, None, None, None);
//!
//! // Send a purchase completed signal
//! client.send(signals::purchase::COMPLETED, Some("user123"), None, None, Some(49.99));
//! ```
//!
//! # Available Signal Categories
//!
//! - `session` - Session lifecycle events
//! - `navigation` - Navigation and routing events
//! - `purchase` - Purchase and monetization events
//! - `acquisition` - User acquisition and onboarding events
//! - `signal` - General signal metadata

/// Session-related signals
pub mod session {
    /// Signal sent when a new session starts
    pub const STARTED: &str = "TelemetryDeck.Session.started";
}

/// Navigation-related signals
pub mod navigation {
    /// Signal sent when navigation path changes
    pub const PATH_CHANGED: &str = "TelemetryDeck.Navigation.pathChanged";
}

/// Purchase-related signals
pub mod purchase {
    /// Signal sent when a purchase is completed
    pub const COMPLETED: &str = "TelemetryDeck.Purchase.completed";
    /// Signal sent when a free trial starts
    pub const FREE_TRIAL_STARTED: &str = "TelemetryDeck.Purchase.freeTrialStarted";
    /// Signal sent when a user converts from trial to paid
    pub const CONVERTED_FROM_TRIAL: &str = "TelemetryDeck.Purchase.convertedFromTrial";
}

/// Acquisition-related signals
pub mod acquisition {
    /// Signal sent when a new install is detected
    pub const NEW_INSTALL_DETECTED: &str = "TelemetryDeck.Acquisition.newInstallDetected";
    /// Signal sent when a lead is started
    pub const LEAD_STARTED: &str = "TelemetryDeck.Acquisition.leadStarted";
    /// Signal sent when a user is acquired
    pub const USER_ACQUIRED: &str = "TelemetryDeck.Acquisition.userAcquired";
    /// Signal sent when a lead converts
    pub const LEAD_CONVERTED: &str = "TelemetryDeck.Acquisition.leadConverted";
}

/// General signal parameters
pub mod signal {
    /// Parameter for signal duration in seconds
    pub const DURATION_IN_SECONDS: &str = "TelemetryDeck.Signal.durationInSeconds";
}
