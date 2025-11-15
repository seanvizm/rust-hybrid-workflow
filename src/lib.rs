// Core modules - only available when not building for web-ui
#[cfg(not(feature = "web-ui"))]
pub mod core;
#[cfg(not(feature = "web-ui"))]
pub mod runners;

// Configuration module
#[cfg(feature = "cli")]
pub mod config;

// Re-export commonly used items - only when core is available
#[cfg(not(feature = "web-ui"))]
pub use core::run_workflow;

#[cfg(feature = "cli")]
pub use config::AppConfig;
