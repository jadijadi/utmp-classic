//! A Rust crate for parsing `utmp` files like `/var/run/utmp` and `/var/log/wtmp`.
//!
//! ## Usage
//!
//! Simplest way is to use `parse_from_*` functions,
//! which returns a `Vec<UtmpEntry>` on success:
//! ```
//! # use anyhow::Result;
//! # fn main() -> Result<()> {
//! let entries = utmp_rs::parse_from_path("/var/run/utmp")?;
//! // ...
//! # Ok(())
//! # }
//! ```
//!
//! If you don't need to collect them all,
//! `UtmpParser` can be used as an iterator:
//! ```
//! # use anyhow::Result;
//! use utmp_rs::UtmpParser;
//! # fn main() -> Result<()> {
//! for entry in UtmpParser::from_path("/var/run/utmp")? {
//!     let entry = entry?;
//!     // ...
//! }
//! # Ok(())
//! # }
//! ```
//!
//! All the `parse_from_*` functions as well as `UtmpParser` parse `utmp` file
//! based on the native format for the target platform.
//! If cross-platform parsing is needed,
//! `Utmp32Parser` or `Utmp64Parser` can be used instead of `UtmpParser`.

mod entry;
mod parse;

pub use entry::{UtmpEntry, UtmpError};
pub use parse::{parse_from_file, parse_from_path, parse_from_reader};
pub use parse::{ParseError, Utmp32Parser, Utmp64Parser, UtmpParser};
