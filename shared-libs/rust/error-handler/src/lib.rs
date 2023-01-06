#![forbid(unsafe_code)]
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::as_conversions)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![allow(clippy::cargo_common_metadata)]
#![allow(clippy::multiple_crate_versions)]
#![allow(clippy::missing_errors_doc)]
pub mod error_mapping;
pub use error_mapping::*;
