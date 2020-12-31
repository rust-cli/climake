//! The simplistic, dependency-free cli library ✨
//!
//! - **[Documentation](https://docs.rs/climake)**
//! - [Crates.io](https://crates.io/crates/climake)
//!
//! # Example 📚
//!
//! Demo of a simple package manager:
//!
//! ```rust
//! use climake::prelude::*;
//!
//! fn main() {
//!     let package = Argument::new(
//!         "The package name",
//!         vec!['p', 'i'],
//!         vec!["pkg, package"],
//!         Input::Text,
//!     );
//!
//!     let add = Subcommand::new("add", vec![&package], vec![], "Adds a package");
//!     let rem = Subcommand::new("rem", vec![&package], vec![], "Removes a package");
//!
//!     let cli = CliMake::new(
//!         "MyPkg",
//!         vec![],
//!         vec![&add, &rem],
//!         "A simple package manager demo",
//!         "1.0.0",
//!     );
//!
//!     let parsed = cli.parse();
//!
//!     for subcommand in parsed.subcommands {
//!         if subcommand.inner == &add {
//!             println!("Adding package {:?}..", subcommand.arguments[0]);
//!         } else if subcommand.inner == &rem {
//!             println!("Removing package {:?}..", subcommand.arguments[0]);
//!         }
//!     }
//! }
//! ```
//!
//! ## Installation 🚀
//!
//! Simply add the following to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! climake = "3.0.0-pre.1" # rewrite isn't out just yet!
//! ```
//!
//! # License
//!
//! This library is duel-licensed under both the [MIT License](https://opensource.org/licenses/MIT)
//! ([`LICENSE-MIT`](https://github.com/rust-cli/climake/blob/master/LICENSE-MIT))
//! and [Apache 2.0 License](https://www.apache.org/licenses/LICENSE-2.0)
//! ([`LICENSE-APACHE`](https://github.com/rust-cli/climake/blob/master/LICENSE-APACHE)),
//! you may choose at your discretion.

#![forbid(unsafe_code)]
#![doc(
    html_logo_url = "https://github.com/rust-cli/climake/raw/master/logo.png",
    html_favicon_url = "https://github.com/rust-cli/climake/raw/master/logo.png"
)]

/// Default help message for [Argument]s without help added
const HELP_DEFAULT: &str = "No help provided";

/// Tabs to render for cli arguments. This will be subtracted from 80 char width
/// of terminals allowed so spaces are reccomended
const CLI_TABBING: &str = "  ";

mod core;

pub mod io;
pub mod parsed;
pub mod prelude;

pub use crate::core::*;
