// Core modules - only available when not building for web-ui
#[cfg(not(feature = "web-ui"))]
pub mod core;
#[cfg(not(feature = "web-ui"))]
pub mod runners;

#[cfg(any(feature = "web-ui", feature = "web-server"))]
pub mod web;

// Re-export commonly used items - only when core is available
#[cfg(not(feature = "web-ui"))]
pub use core::run_workflow;
