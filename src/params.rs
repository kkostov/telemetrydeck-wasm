//! Reserved parameter name constants defined by TelemetryDeck
//!
//! This module provides pre-defined parameter names for common telemetry data.
//! Using these constants ensures your signals are properly recognized by
//! TelemetryDeck's analytics engine and can be used in built-in dashboards.
//!
//! # Example
//!
//! ```no_run
//! use telemetrydeck_wasm::{TelemetryDeck, params};
//! use std::collections::HashMap;
//!
//! let client = TelemetryDeck::new("YOUR-APP-ID");
//!
//! let mut payload = HashMap::new();
//! payload.insert(params::device::PLATFORM.to_string(), "iOS".to_string());
//! payload.insert(params::device::SYSTEM_VERSION.to_string(), "17.0".to_string());
//! payload.insert(params::user_preferences::LANGUAGE.to_string(), "en".to_string());
//!
//! client.send("appOpened", Some("user"), Some(payload), None, None);
//! ```
//!
//! # Available Parameter Categories
//!
//! - `accessibility` - Accessibility settings (font scale, reduced motion, etc.)
//! - `acquisition` - User acquisition data (first session, channel, etc.)
//! - `device` - Device information (platform, OS, architecture, etc.)
//! - `navigation` - Navigation paths and routes
//! - `purchase` - Purchase details (type, price, currency, etc.)
//! - `retention` - User retention metrics (session count, duration, etc.)
//! - `calendar` - Time-based information (day, week, month, etc.)
//! - `run_context` - Runtime environment (locale, marketplace, etc.)
//! - `user_preferences` - User preferences (language, color scheme, etc.)

/// Accessibility-related parameters
pub mod accessibility {
    /// Font weight adjustment setting
    pub const FONT_WEIGHT_ADJUSTMENT: &str = "TelemetryDeck.Accessibility.fontWeightAdjustment";
    /// Font scale setting
    pub const FONT_SCALE: &str = "TelemetryDeck.Accessibility.fontScale";
    /// Whether bold text is enabled
    pub const IS_BOLD_TEXT_ENABLED: &str = "TelemetryDeck.Accessibility.isBoldTextEnabled";
    /// Whether darker system colors are enabled
    pub const IS_DARKER_SYSTEM_COLORS_ENABLED: &str =
        "TelemetryDeck.Accessibility.isDarkerSystemColorsEnabled";
    /// Whether invert colors is enabled
    pub const IS_INVERT_COLORS_ENABLED: &str = "TelemetryDeck.Accessibility.isInvertColorsEnabled";
    /// Whether reduce motion is enabled
    pub const IS_REDUCE_MOTION_ENABLED: &str = "TelemetryDeck.Accessibility.isReduceMotionEnabled";
    /// Whether reduce transparency is enabled
    pub const IS_REDUCE_TRANSPARENCY_ENABLED: &str =
        "TelemetryDeck.Accessibility.isReduceTransparencyEnabled";
    /// Whether to differentiate without color
    pub const SHOULD_DIFFERENTIATE_WITHOUT_COLOR: &str =
        "TelemetryDeck.Accessibility.shouldDifferentiateWithoutColor";
}

/// Acquisition-related parameters
pub mod acquisition {
    /// Date of first session
    pub const FIRST_SESSION_DATE: &str = "TelemetryDeck.Acquisition.firstSessionDate";
    /// Acquisition channel
    pub const CHANNEL: &str = "TelemetryDeck.Acquisition.channel";
    /// Lead identifier
    pub const LEAD_ID: &str = "TelemetryDeck.Acquisition.leadID";
}

/// Device-related parameters
pub mod device {
    /// Device architecture (e.g., arm64, x86_64)
    pub const ARCHITECTURE: &str = "TelemetryDeck.Device.architecture";
    /// Device model name
    pub const MODEL_NAME: &str = "TelemetryDeck.Device.modelName";
    /// Operating system name
    pub const OPERATING_SYSTEM: &str = "TelemetryDeck.Device.operatingSystem";
    /// Platform name
    pub const PLATFORM: &str = "TelemetryDeck.Device.platform";
    /// System version (major.minor)
    pub const SYSTEM_MAJOR_MINOR_VERSION: &str = "TelemetryDeck.Device.systemMajorMinorVersion";
    /// System major version
    pub const SYSTEM_MAJOR_VERSION: &str = "TelemetryDeck.Device.systemMajorVersion";
    /// Full system version
    pub const SYSTEM_VERSION: &str = "TelemetryDeck.Device.systemVersion";
    /// Device brand
    pub const BRAND: &str = "TelemetryDeck.Device.brand";
    /// Device timezone
    pub const TIME_ZONE: &str = "TelemetryDeck.Device.timeZone";
    /// Device orientation
    pub const ORIENTATION: &str = "TelemetryDeck.Device.orientation";
    /// Screen density
    pub const SCREEN_DENSITY: &str = "TelemetryDeck.Device.screenDensity";
    /// Screen height in pixels
    pub const SCREEN_HEIGHT: &str = "TelemetryDeck.Device.screenResolutionHeight";
    /// Screen width in pixels
    pub const SCREEN_WIDTH: &str = "TelemetryDeck.Device.screenResolutionWidth";
}

/// Navigation-related parameters
pub mod navigation {
    /// Navigation schema version
    pub const SCHEMA_VERSION: &str = "TelemetryDeck.Navigation.schemaVersion";
    /// Navigation identifier
    pub const IDENTIFIER: &str = "TelemetryDeck.Navigation.identifier";
    /// Source path
    pub const SOURCE_PATH: &str = "TelemetryDeck.Navigation.sourcePath";
    /// Destination path
    pub const DESTINATION_PATH: &str = "TelemetryDeck.Navigation.destinationPath";
}

/// Purchase-related parameters
pub mod purchase {
    /// Purchase type
    pub const TYPE: &str = "TelemetryDeck.Purchase.type";
    /// Country code
    pub const COUNTRY_CODE: &str = "TelemetryDeck.Purchase.countryCode";
    /// Currency code
    pub const CURRENCY_CODE: &str = "TelemetryDeck.Purchase.currencyCode";
    /// Product identifier
    pub const PRODUCT_ID: &str = "TelemetryDeck.Purchase.productID";
    /// Offer identifier
    pub const OFFER_ID: &str = "TelemetryDeck.Purchase.offerID";
    /// Price in micros
    pub const PRICE_MICROS: &str = "TelemetryDeck.Purchase.priceMicros";
}

/// Retention-related parameters
pub mod retention {
    /// Average session duration in seconds
    pub const AVERAGE_SESSION_SECONDS: &str = "TelemetryDeck.Retention.averageSessionSeconds";
    /// Distinct days used
    pub const DISTINCT_DAYS_USED: &str = "TelemetryDeck.Retention.distinctDaysUsed";
    /// Total sessions count
    pub const TOTAL_SESSIONS_COUNT: &str = "TelemetryDeck.Retention.totalSessionsCount";
    /// Previous session duration in seconds
    pub const PREVIOUS_SESSION_SECONDS: &str = "TelemetryDeck.Retention.previousSessionSeconds";
    /// Distinct days used in last month
    pub const DISTINCT_DAYS_USED_LAST_MONTH: &str =
        "TelemetryDeck.Retention.distinctDaysUsedLastMonth";
}

/// Calendar-related parameters
pub mod calendar {
    /// Day of month (1-31)
    pub const DAY_OF_MONTH: &str = "TelemetryDeck.Calendar.dayOfMonth";
    /// Day of week (1-7)
    pub const DAY_OF_WEEK: &str = "TelemetryDeck.Calendar.dayOfWeek";
    /// Day of year (1-366)
    pub const DAY_OF_YEAR: &str = "TelemetryDeck.Calendar.dayOfYear";
    /// Week of year (1-53)
    pub const WEEK_OF_YEAR: &str = "TelemetryDeck.Calendar.weekOfYear";
    /// Whether it's a weekend
    pub const IS_WEEKEND: &str = "TelemetryDeck.Calendar.isWeekend";
    /// Month of year (1-12)
    pub const MONTH_OF_YEAR: &str = "TelemetryDeck.Calendar.monthOfYear";
    /// Quarter of year (1-4)
    pub const QUARTER_OF_YEAR: &str = "TelemetryDeck.Calendar.quarterOfYear";
    /// Hour of day (0-23)
    pub const HOUR_OF_DAY: &str = "TelemetryDeck.Calendar.hourOfDay";
}

/// Run context parameters
pub mod run_context {
    /// Locale setting
    pub const LOCALE: &str = "TelemetryDeck.RunContext.locale";
    /// Target environment
    pub const TARGET_ENVIRONMENT: &str = "TelemetryDeck.RunContext.targetEnvironment";
    /// Whether app is side-loaded
    pub const IS_SIDE_LOADED: &str = "TelemetryDeck.RunContext.isSideLoaded";
    /// Source marketplace
    pub const SOURCE_MARKETPLACE: &str = "TelemetryDeck.RunContext.sourceMarketplace";
}

/// User preference parameters
pub mod user_preferences {
    /// Layout direction (LTR/RTL)
    pub const LAYOUT_DIRECTION: &str = "TelemetryDeck.UserPreference.layoutDirection";
    /// Region setting
    pub const REGION: &str = "TelemetryDeck.UserPreference.region";
    /// Language setting
    pub const LANGUAGE: &str = "TelemetryDeck.UserPreference.language";
    /// Color scheme preference
    pub const COLOR_SCHEME: &str = "TelemetryDeck.UserPreference.colorScheme";
}
