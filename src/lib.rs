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

mod utils;

pub mod io;
pub mod parsed;
pub mod prelude;

use std::io::{prelude::*, LineWriter};
use std::{env, fmt};

/// Default help message for [Argument]s without help added
const HELP_DEFAULT: &str = "No help provided";

/// Tabs to render for cli arguments. This will be subtracted from 80 char width
/// of terminals allowed so spaces are reccomended
const CLI_TABBING: &str = "  ";

/// A single type of call for an [Argument], can be a short call or a long call
#[derive(Debug, PartialEq, Clone)]
enum CallType {
    /// Short, single-char call, e.g. `-h`
    Short(char),

    /// Long, multi-char call, e.g. `--hello`
    Long(String),
}

impl fmt::Display for CallType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CallType::Short(c) => write!(f, "{}", c),
            CallType::Long(string) => write!(f, "--{}", string),
        }
    }
}

impl From<CallType> for String {
    fn from(calltype: CallType) -> Self {
        match calltype {
            CallType::Short(c) => String::from(c),
            CallType::Long(string) => string,
        }
    }
}

impl From<char> for CallType {
    fn from(c: char) -> Self {
        CallType::Short(c)
    }
}

impl From<String> for CallType {
    fn from(string: String) -> Self {
        CallType::Long(string)
    }
}

/// An argument attached to the cli, allowing passing of user data to the top-level
/// cli or subcommands
#[derive(Debug, PartialEq, Clone)]
pub struct Argument<'a> {
    /// Optional help message
    help: Option<&'a str>,

    /// Many [CallType]s corrosponding to this argument
    calls: Vec<CallType>,

    /// [io::Input] type allowed for this argument
    input: io::Input,

    /// Required argument for given root cli or [Subcommand]. If this argument is
    /// not present whilst the cli parses, it will provide an apt error
    ///
    /// To change the default behaviour of `false` (not required), simply modify
    /// this value before it's time to parse.
    required: bool,
}

impl<'a> Argument<'a> {
    /// Creates a new [Argument] from given passed values
    pub fn new(
        help: impl Into<Option<&'a str>>,
        short_calls: impl IntoIterator<Item = char>,
        long_calls: impl IntoIterator<Item = &'a str>,
        input: impl Into<io::Input>,
    ) -> Self {
        let mut calls: Vec<CallType> = short_calls
            .into_iter()
            .map(|call| CallType::Short(call))
            .collect();
        calls.append(
            &mut long_calls
                .into_iter()
                .map(|call| CallType::Long(call.to_string()))
                .collect::<Vec<CallType>>(),
        );

        Self {
            help: help.into(),
            calls,
            input: input.into(),
            required: false,
        }
    }

    /// Adds a single short call, chainable
    pub fn add_scall(&mut self, short_call: impl Into<char>) -> &mut Self {
        self.calls.push(short_call.into().into());
        self
    }

    /// Adds multiple short calls, chainable
    pub fn add_scalls(&mut self, short_calls: impl IntoIterator<Item = char>) -> &mut Self {
        for c in short_calls.into_iter() {
            self.add_scall(c);
        }
        self
    }

    /// Adds a single long call, chainable
    pub fn add_lcall(&mut self, long_call: impl Into<String>) -> &mut Self {
        self.calls.push(long_call.into().into());
        self
    }

    /// Adds multiple long calls, chainable
    pub fn add_lcalls(&mut self, long_calls: impl IntoIterator<Item = String>) -> &mut Self {
        for c in long_calls.into_iter() {
            self.add_lcall(c);
        }
        self
    }

    /// Generates compact help message for current [Argument]
    ///
    /// This writes directly to a buffer of some kind (typically [std::io::stdout])
    /// for simplicity, perf and extendability reasons.
    ///
    /// # Example
    ///
    /// What this may look like:
    ///
    /// ```none
    ///   (-v, --verbose) — Verbose mode
    /// ```
    fn help_name_msg(&self, buf: &mut impl Write) -> std::io::Result<()> {
        let mut lc_buf: Vec<String> = Vec::new();
        let mut sc_buf: Vec<char> = Vec::new();

        for call in self.calls.iter() {
            match call {
                CallType::Long(call) => lc_buf.push(format!("--{}", call)),
                CallType::Short(call) => sc_buf.push(*call),
            }
        }

        let short_calls: String = if sc_buf.len() == 0 {
            String::new()
        } else {
            format!("-{}", sc_buf.iter().collect::<String>())
        };

        let mut formatted_calls = vec![short_calls];
        formatted_calls.append(&mut lc_buf);

        let formatted_help = match self.help {
            Some(msg) => msg,
            None => HELP_DEFAULT,
        };
        let required_msg = if self.required { "[REQUIRED] " } else { "" };

        utils::writeln_term(
            if formatted_calls.len() == 1 && formatted_calls[0] != "" {
                format!(
                    "{} {}{}— {}",
                    formatted_calls[0], self.input, required_msg, formatted_help
                )
            } else {
                format!(
                    "({}) {}{}— {}",
                    formatted_calls.join(", "),
                    self.input,
                    required_msg,
                    formatted_help,
                )
            },
            buf,
        )
    }
}

/// A subcommand attached to the cli, allowing commands and sections of the cli
/// to form
#[derive(Debug, PartialEq, Clone)]
pub struct Subcommand<'a> {
    /// Name of subcommand, used both in help and as the single calling method
    pub name: &'a str,

    /// Argument(s) attached to this [Subcommand], if any
    pub arguments: Vec<&'a Argument<'a>>,

    /// Recursive subcommands attached to this [Subcommand], if any
    pub subcommands: Vec<&'a Subcommand<'a>>,

    /// Optional short description of this subcommand
    pub help: Option<&'a str>,
}

impl<'a> Subcommand<'a> {
    /// Creates a new subcommand from given abstracted inputs
    pub fn new(
        name: impl Into<&'a str>,
        arguments: impl Into<Vec<&'a Argument<'a>>>,
        subcommands: impl Into<Vec<&'a Subcommand<'a>>>,
        help: impl Into<Option<&'a str>>,
    ) -> Self {
        Self {
            name: name.into(),
            arguments: arguments.into(),
            subcommands: subcommands.into(),
            help: help.into(),
        }
    }

    /// Displays help infomation for this subcommand specifically which is used
    /// inside the execution of the cli
    ///
    /// A referenced [CliMake] is needed for this method due to it displaying a
    /// header message using [CliMake::header_msg] with an altered usage line, as
    /// seen in the examples.
    fn help_msg(&self, climake: &CliMake, buf: &mut impl Write) -> std::io::Result<()> {
        climake.header_msg(self.name, buf)?;

        match self.help {
            Some(help) => {
                buf.write("\nAbout:\n".as_bytes())?;
                utils::writeln_term(help, buf)?;
            }
            None => (),
        };

        // TODO: merge this into a utility func shared with CliMake::help_msg
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

    /// Generates compact help message for current [Subcommand]
    ///
    /// This writes directly to a buffer of some kind (typically [std::io::stdout])
    /// for simplicity, perf and extendability reasons.
    ///
    /// # Example
    ///
    /// What this may look like:
    ///
    /// ```none
    ///   example — A simple example subcommand
    /// ```
    fn help_name_msg(&self, buf: &mut impl Write) -> std::io::Result<()> {
        let formatted_help = match self.help {
            Some(msg) => msg,
            None => HELP_DEFAULT,
        };

        utils::writeln_term(format!("{} — {}", self.name, formatted_help), buf)
    }
}

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
    fn header_msg(
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
    fn help_msg(&self, buf: &mut impl Write) -> std::io::Result<()> {
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
    /// default parsing from [std::os::args]
    pub fn parse_custom(
        &'a self,
        arguments: impl IntoIterator<Item = String>,
    ) -> parsed::ParsedCli<'a> {
        // for argument in arguments.into_iter() {}
        unimplemented!()
    }

    /// Parses default arguments coming from [std::os::args]
    pub fn parse(&'a self) -> parsed::ParsedCli<'a> {
        self.parse_custom(env::args())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that the [Argument::new] method (creation of arguments) works correctly
    #[test]
    fn arg_new() {
        assert_eq!(
            Argument::new(None, vec!['a', 'b'], vec!["hi", "there"], io::Input::Text),
            Argument {
                calls: vec![
                    CallType::Short('a'),
                    CallType::Short('b'),
                    CallType::Long("hi".to_string()),
                    CallType::Long("there".to_string())
                ],
                help: None,
                input: io::Input::Text,
                required: false,
            }
        )
    }

    /// Checks that the [Argument::help_name_msg] method works correctly
    #[test]
    fn arg_name_help() -> std::io::Result<()> {
        let mut chk_vec: Vec<u8> = vec![];

        Argument::new(None, vec![], vec![], io::Input::None).help_name_msg(&mut chk_vec)?;
        assert_eq!(
            std::str::from_utf8(chk_vec.as_slice()).unwrap(),
            "  () — No help provided\n"
        );
        chk_vec = vec![];

        Argument::new("Some simple help", vec!['a'], vec!["long"], io::Input::Text)
            .help_name_msg(&mut chk_vec)?;
        assert_eq!(
            std::str::from_utf8(chk_vec.as_slice()).unwrap(),
            "  (-a, --long) [text] — Some simple help\n"
        );
        chk_vec = vec![];

        Argument::new(None, vec!['a'], vec![], io::Input::Text).help_name_msg(&mut chk_vec)?;
        assert_eq!(
            std::str::from_utf8(chk_vec.as_slice()).unwrap(),
            "  -a [text] — No help provided\n"
        );

        Ok(())
    }

    /// Checks that the [Argument::help_name_msg] method works correctly with [Argument::required]
    /// set to `true`
    #[test]
    fn arg_name_help_required() -> std::io::Result<()> {
        let mut chk_vec: Vec<u8> = vec![];

        let mut arg = Argument::new("Some argument", vec!['s'], vec![], io::Input::None);
        arg.required = true;
        arg.help_name_msg(&mut chk_vec)?;
        assert_eq!(
            std::str::from_utf8(chk_vec.as_slice()).unwrap(),
            "  -s [REQUIRED] — Some argument\n"
        );

        Ok(())
    }

    /// Checks that the [Subcommand::help_name_msg] method works correctly
    #[test]
    fn subcommand_name_help() -> std::io::Result<()> {
        let mut chk_vec: Vec<u8> = vec![];

        Subcommand::new("command", vec![], vec![], "A simple command")
            .help_name_msg(&mut chk_vec)?;
        assert_eq!(
            std::str::from_utf8(chk_vec.as_slice()).unwrap(),
            "  command — A simple command\n"
        );

        Ok(())
    }

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

    /// Checks that the [Argument::add_scall] method works correctly
    #[test]
    fn arg_add_scall() {
        let mut arg = Argument::new("example", vec![], vec![], io::Input::None);

        arg.add_scall('a').add_scall('b').add_scall('c');

        assert_eq!(
            arg,
            Argument::new("example", vec!['a', 'b', 'c'], vec![], io::Input::None)
        )
    }

    /// Checks that the [Argument::add_scalls] method works correctly
    #[test]
    fn arg_add_scalls() {
        let mut arg = Argument::new("example", vec![], vec![], io::Input::None);

        arg.add_scalls(vec!['a', 'b']).add_scalls(vec!['c']);

        assert_eq!(
            arg,
            Argument::new("example", vec!['a', 'b', 'c'], vec![], io::Input::None)
        )
    }

    /// Checks that the [Argument::add_lcall] method works correctly
    #[test]
    fn arg_add_lcall() {
        let mut arg = Argument::new("example", vec![], vec![], io::Input::None);

        arg.add_lcall("a").add_lcall("b").add_lcall("c");

        assert_eq!(
            arg,
            Argument::new("example", vec![], vec!["a", "b", "c"], io::Input::None)
        )
    }

    /// Checks that the [Argument::add_lcalls] method works correctly
    #[test]
    fn arg_add_lcalls() {
        let mut arg = Argument::new("example", vec![], vec![], io::Input::None);

        arg.add_lcalls(vec!["a".to_string(), "b".to_string()])
            .add_lcalls(vec!["c".to_string()]);

        assert_eq!(
            arg,
            Argument::new("example", vec![], vec!["a", "b", "c"], io::Input::None)
        )
    }

    /// Checks that the [From]<[CallType]> implementation for [String] works correctly
    #[test]
    fn string_from_calltype() {
        assert_eq!(String::from(CallType::Short('h')), "h".to_string());
        assert_eq!(
            String::from(CallType::Long("testing".to_string())),
            "testing".to_string()
        );
    }
}
