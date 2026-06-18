//! Interactive Brokers ForecastTrader venue metadata.
//!
//! The execution core, risk layer, and strategy implementations live in the
//! shared engine crate. This module just describes the venue this binary targets.

/// Display name of this venue.
pub const NAME: &str = "Interactive Brokers ForecastTrader";

/// Venue category.
pub const VENUE_TYPE: &str = "Financial events";

/// Strategies this venue runs on the shared engine.
pub const STRATEGIES: &[&str] = &[
    "Resolution Sniper",
    "Spread Farming",
    "Market Making",
];
