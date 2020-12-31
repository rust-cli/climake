//! The simplistic, dependency-free cli library ✨
//!
//! - **[Documentation](https://docs.rs/climake)**
//! - [Crates.io](https://crates.io/crates/climake)
//!
//! # Example 📚
//!
//! Rewrite example coming soon!
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

mod argument;
mod call_type;
mod subcommand;
mod utils;

pub mod io;
pub mod parsed;
pub mod prelude;

pub use crate::argument::Argument;
pub use crate::subcommand::Subcommand;

use std::env;
use std::io::prelude::*;

/// Default help message for [Argument]s without help added
const HELP_DEFAULT: &str = "No help provided";

/// Tabs to render for cli arguments. This will be subtracted from 80 char width
/// of terminals allowed so spaces are reccomended
const CLI_TABBING: &str = "  ";

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

    /// Internal/private tabbing to use, defaults to [CLI_TABBING]
    tabbing: &'static str,
}

impl<'a> CliMake<'a> {
    /// Creates a new [Argument] from given passed values
    pub fn new(
        name: impl Into<&'a str>,
        arguments: impl Into<Vec<&'a Argument<'a>>>,
        subcommands: impl Into<Vec<&'a Subcommand<'a>>>,
        description: impl Into<Option<&'a str>>,
        version: impl Into<Option<&'a str>>,
    ) -> Self {
        CliMake {
            name: name.into(),
            arguments: arguments.into(),
            subcommands: subcommands.into(),
            description: description.into(),
            version: version.into(),
            tabbing: CLI_TABBING,
        }
    }

    /// Adds a single argument to this root [CliMake], chainable
    pub fn add_arg(&mut self, argument: impl Into<&'a Argument<'a>>) -> &mut Self {
        self.arguments.push(argument.into());
        self
    }

    /// Adds multiple arguments to this root [CliMake], chainable
    pub fn add_args(&mut self, arguments: impl IntoIterator<Item = &'a Argument<'a>>) -> &mut Self {
        for arg in arguments.into_iter() {
            self.add_arg(arg);
        }
        self
    }

    /// Adds a single subcommand to this root [CliMake], chainable
    pub fn add_subcmd(&mut self, subcommand: impl Into<&'a Subcommand<'a>>) -> &mut Self {
        self.subcommands.push(subcommand.into());
        self
    }

    /// Adds multiple subcommands to this root [CliMake], chainable
    pub fn add_subcmds(
        &mut self,
        subcommands: impl IntoIterator<Item = &'a Subcommand<'a>>,
    ) -> &mut Self {
        for subcommand in subcommands.into_iter() {
            self.add_subcmd(subcommand);
        }
        self
    }

    /// Sets the tabbing characters for cli help, the default for this is 2 spaces,
    /// i.e. `  `.
    pub fn tabbing(&mut self, tab_chars: &'static str) -> &mut Self {
        self.tabbing = tab_chars;
        self
    }

    /// Generates header and streams to given [Write] buffer for displaying info
    /// about this cli.
    ///
    /// Please check [CliMake::help_msg] for the full help message generation used
    /// throughout automatic execution of this cli. The `usage_suffix` input used
    /// for this method is used for [Subcommand] help where the subcommand in
    /// question would like to display itself on the end of the top usage line
    /// for the header
    ///
    /// # Example
    ///
    /// What this may display:
    ///
    /// ```none
    /// Usage: ./my-app [OPTIONS]
    ///
    ///   My app v0.1.0 — A simple application
    /// ```
    pub(crate) fn header_msg(
        &self,
        usage_suffix: impl Into<Option<&'a str>>,
        buf: &mut impl Write,
    ) -> std::io::Result<()> {
        let cur_exe = env::current_exe().unwrap(); // TODO: better errors
        let cur_stem = cur_exe.file_stem().unwrap().to_str().unwrap(); // TOOD: better errors

        match usage_suffix.into() {
            Some(suffix) => {
                buf.write_fmt(format_args!("Usage: ./{} {} [OPTIONS]\n", cur_stem, suffix))?
            }
            None => buf.write_fmt(format_args!("Usage: ./{} [OPTIONS]\n", cur_stem))?,
        }

        match self.description.clone() {
            Some(d) => {
                buf.write("\n".as_bytes())?; // write formatting empty byte

                utils::writeln_term(
                    match &self.version {
                        Some(v) => format!("{} v{} — {}", self.name, v, d),
                        None => format!("{} — {}", self.name, d),
                    },
                    buf,
                )
            }
            None => Ok(()),
        }
    }

    /// Displays help infomation for climake which is used inside the execution
    /// of the cli
    ///
    /// # Help sources
    ///
    /// This method gets sections of messaging such as the header from various
    /// *public*-available methods inside of this library:
    ///
    /// - [CliMake::header_msg]: Header generation for help message and errors
    /// - [Argument::help_name_msg]: Help generation for single [Argument]s
    ///
    /// # Example
    ///
    /// What this may look like:
    ///
    /// ```none
    /// Usage: ./my-app [OPTIONS]
    ///
    ///   My app v0.1.0 — A simple application
    ///
    /// Arguments:
    ///   (-v, --verbose) — Verbose mode
    /// ```
    pub(crate) fn help_msg(&self, buf: &mut impl Write) -> std::io::Result<()> {
        self.header_msg(None, buf)?;

        buf.write("\nArguments:\n".as_bytes())?;

        if self.arguments.len() > 0 {
            for argument in self.arguments.iter() {
                argument.help_name_msg(buf)?;
            }
        } else {
            buf.write("  No arguments found\n".as_bytes())?;
        }

        buf.write("\nSubcommands:\n".as_bytes())?;

        if self.subcommands.len() > 0 {
            for subcommand in self.subcommands.iter() {
                subcommand.help_name_msg(buf)?;
            }
        } else {
            buf.write("  No subcommands found\n".as_bytes())?;
        }

        Ok(())
    }

    /// Parses all arguments from a custom iterator, see [CliMake::parse] for
    /// default parsing from [env::args]
    pub fn parse_custom(
        &'a self,
        arguments: impl IntoIterator<Item = String>,
    ) -> parsed::ParsedCli<'a> {
        // for argument in arguments.into_iter() {}
        unimplemented!()
    }

    /// Parses default arguments coming from [env::args]
    pub fn parse(&'a self) -> parsed::ParsedCli<'a> {
        self.parse_custom(env::args())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that the [CliMake::add_arg] method works correctly
    #[test]
    fn cli_add_arg() {
        let mut cli = CliMake::new("example", vec![], vec![], "Add arg check", None);
        let arg = Argument::new("arg help", vec![], vec![], io::Input::None);

        cli.add_arg(&arg).add_arg(&arg);

        assert_eq!(cli.arguments, vec![&arg, &arg])
    }

    /// Checks that the [CliMake::add_args] method works correctly
    #[test]
    fn cli_add_args() {
        let mut cli = CliMake::new("example", vec![], vec![], "Add arg check", None);
        let arg = Argument::new("arg help", vec![], vec![], io::Input::None);

        cli.add_args(vec![&arg, &arg]).add_args(vec![&arg, &arg]);

        assert_eq!(cli.arguments, vec![&arg, &arg, &arg, &arg])
    }

    /// Checks that the [CliMake::add_subcmds] method works correctly
    #[test]
    fn cli_add_subcmds() {
        let mut cli = CliMake::new("example", vec![], vec![], "Add arg check", None);
        let subcmd = Subcommand::new("example", vec![], vec![], None);

        cli.add_subcmds(vec![&subcmd, &subcmd])
            .add_subcmds(vec![&subcmd, &subcmd]);

        assert_eq!(cli.subcommands, vec![&subcmd, &subcmd, &subcmd, &subcmd])
    }

    /// Checks that the [CliMake::add_subcmd] method works correctly
    #[test]
    fn cli_add_subcmd() {
        let mut cli = CliMake::new("example", vec![], vec![], "Add arg check", None);
        let subcmd = Subcommand::new("example", vec![], vec![], None);

        cli.add_subcmd(&subcmd).add_subcmd(&subcmd);

        assert_eq!(cli.subcommands, vec![&subcmd, &subcmd])
    }
}
