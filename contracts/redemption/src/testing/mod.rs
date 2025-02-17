// Conditionally include the `integration` tests module
#[cfg(feature = "integration")]
mod integration;

// Conditionally include the `unit` tests module
#[cfg(feature = "unit")]
mod unit;

// Include all test modules if no specific feature is specified
#[cfg(not(any(feature = "unit", feature = "integration")))]
mod integration;

#[cfg(not(any(feature = "unit", feature = "integration")))]
mod unit;
