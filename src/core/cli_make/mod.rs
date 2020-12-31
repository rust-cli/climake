//! Contains [CliMake]-related items, see specific documentation for more information

mod impl_basic;
mod impl_help;
mod impl_parse;

pub use impl_basic::*;
pub use impl_help::*;
pub use impl_parse::*;

use crate::{Argument, Subcommand};

/// The core climake structure, facilitating creation and parsing of both arguments
/// and subcommands
#[derive(Debug, PartialEq, Clone)]
pub struct CliMake<'a> {
    /// Name of the program using the cli
    name: &'a str,

    /// Internal [Argument]s stored inside the cli once created/added to
    arguments: Vec<&'a Argument<'a>>,

    /// Internal [Subcommand]s stored inside the cli once created/added to
    subcommands: Vec<&'a Subcommand<'a>>,

    /// Optional short description of the program using the cli
    description: Option<&'a str>,

    /// Optional version string of the program using the cli
    ///
    /// # Crate version
    ///
    /// If you would like this value to automatically update with your crates version,
    /// you may use a variation of the following function:
    ///
    /// ```rust
    /// pub fn crate_version() -> String {
    ///     format!(
    ///         "{}.{}.{}{}",
    ///         env!("CARGO_PKG_VERSION_MAJOR"),
    ///         env!("CARGO_PKG_VERSION_MINOR"),
    ///         env!("CARGO_PKG_VERSION_PATCH"),
    ///         option_env!("CARGO_PKG_VERSION_PRE").unwrap_or("")
    ///     )
    /// }
    /// ```
    version: Option<&'a str>,

    /// Internal/private tabbing to use, defaults to [CLI_TABBING](crate::CLI_TABBING)
    tabbing: &'static str,
}
