// Conditionally include the `integration` tests module
#[cfg(feature = "integration")]
mod integration;

// Conditionally include the `migration` tests module
#[cfg(feature = "migration")]
mod migration;

// Include all test modules if no specific feature is specified
#[cfg(not(any(feature = "unit", feature = "integration", feature = "migration")))]
mod integration;
#[cfg(not(any(feature = "unit", feature = "integration", feature = "migration")))]
mod migration;
