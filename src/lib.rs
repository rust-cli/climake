//! The simple, dependency-free cli library ✨
//!
//! - [Crates.io](https://crates.io/crates/climake)
//! - [Documentation](https://docs.rs/climake)
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
//! climake = "3.0" # note: rewrite isn't out just yet!
//! ```
//!
//! # License
//!
//! Duel-licensed under both the [MIT License](https://opensource.org/licenses/MIT)
//! ([`LICENSE-MIT`](LICENSE-MIT)) and [Apache 2.0 License](https://www.apache.org/licenses/LICENSE-2.0)
//! ([`LICENSE-APACHE`](LICENSE-APACHE)), you may choose at your discretion.

#![forbid(unsafe_code)]
#![doc(
    html_logo_url = "https://github.com/rust-cli/climake/raw/master/logo.png",
    html_favicon_url = "https://github.com/rust-cli/climake/raw/master/logo.png"
)]

use std::io::{prelude::*, LineWriter};
use std::path::PathBuf;
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

/// An input type, typically given for an [Argument] to descibe what types are
/// allowed to be passwed in. This is then transferred to [Data] once the cli
/// has been executed
#[derive(Debug, PartialEq, Clone)]
pub enum Input {
    /// No input allowed, will error if any is given. Maps to [Data::None]
    None,

    /// Text input allowed, this will return an empty string if no text is supplied.
    /// Maps to [Data::Text]
    Text,

    /// A single [PathBuf] given to the argument, these are not certain to exist
    /// and simply echo the user's input. Maps to [Data::Path]
    Path,

    /// Multiple [PathBuf]s given to the argument, these are not certain to exist
    /// and simply echo the user's input. Maps to [Data::Paths]
    Paths,
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // formatting has a space on existing words on purpouse for help generation
        match self {
            Input::None => write!(f, ""),
            Input::Text => write!(f, "[text] "),
            Input::Path => write!(f, "[path] "),
            Input::Paths => write!(f, "[paths] "),
        }
    }
}

/// Outputted data from parsing a cli for each argument. This enumeration is based
/// upon the allowed [Input] of a given [Argument] and maps directly to the input
///
/// # Mappings from [Input]
///
/// If a user requested for an [Argument] to be of [Input::Path],
/// once parsed this enumeration would be [Data::Path] (in corrospondance with
/// the name).
#[derive(Debug, PartialEq, Clone)]
pub enum Data {
    /// No data provided, from [Input::None]
    None,

    /// Textual input provided, from [Input::Text]. This may be an empty string
    /// in the case of the user not actually providing input
    Text(String),

    /// Path input provided, from [Input::Path]. This may be an empty or invalid
    /// [PathBuf] in the case of user input being misleading or non-existant
    Path(PathBuf),

    /// Multiple path inputs provided, from [Input::Paths]. This may be an empty
    /// vector (i.e. length 0) if the user doesn't provide any paths or may be
    /// non-existant paths given from user input
    Paths(Vec<PathBuf>),
}

impl Data {
    /// Creates a new [Data] from with types mapping from [Input] using passed
    /// `data`. This may map the `data` string vec into types such as `PathBuf`
    pub fn new(input: Input, data: impl IntoIterator<Item = String>) -> Self {
        match input {
            Input::None => Data::None, // ignore passed `data` (if any)
            Input::Text => match data.into_iter().next() {
                Some(text) => Data::Text(text),
                None => Data::Text(String::new()),
            },
            Input::Path => match data.into_iter().next() {
                Some(path_string) => Data::Path(PathBuf::from(path_string)),
                None => Data::Path(PathBuf::new()),
            },
            Input::Paths => Data::Paths(
                data.into_iter()
                    .map(|path_string| PathBuf::from(path_string))
                    .collect(),
            ),
        }
    }
}

/// An argument, infomaton coming soon..
#[derive(Debug, PartialEq, Clone)]
pub struct Argument<'a> {
    /// Optional help message
    help: Option<&'a str>,

    /// Many [CallType]s corrosponding to this argument
    calls: Vec<CallType>,

    /// [Input] type allowed for this argument
    input: Input,

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
        input: impl Into<Input>,
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

    /// Generates compact help message for current [Argument]
    ///
    /// This writes directly to a buffer of some kind (typically [std::io::stdout])
    /// for simplicity, perf and extendability reasons.
    ///
    /// # Example
    ///
    /// Usage:
    ///
    /// ```rust
    /// use std::io;
    /// use climake::{Argument, Input};
    ///
    /// fn main() {
    ///     let arg = Argument::new(
    ///         "Verbose mode", vec!['v'], vec!["verbose"], Input::None
    ///     );
    ///
    ///     arg.help_name_msg(&mut io::stdout()).unwrap();
    /// }
    /// ```
    ///
    /// What this may look like:
    ///
    /// ```none
    ///   (-v, --verbose) — Verbose mode
    /// ```
    pub fn help_name_msg(&self, buf: &mut impl Write) -> std::io::Result<()> {
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

        writeln_term(
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

/// Subcommand attached to a cli, allowing non-argument commands to be executed
/// with arguments attached to oneself for more complex operations
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
    pub fn help_msg(&self, climake: &CliMake, buf: &mut impl Write) -> std::io::Result<()> {
        climake.header_msg(self.name, buf)?;

        match self.help {
            Some(help) => {
                buf.write("\nAbout:\n".as_bytes())?;
                writeln_term(help, buf)?;
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
    /// Usage:
    ///
    /// ```rust
    /// use std::io;
    /// use climake::{Subcommand, Input};
    ///
    /// fn main() {
    ///     let subcmd = Subcommand::new(
    ///         "example", vec![], vec![], "A simple example subcommand"
    ///     );
    ///
    ///     subcmd.help_name_msg(&mut io::stdout()).unwrap();
    /// }
    /// ```
    ///
    /// What this may look like:
    ///
    /// ```none
    ///   example — A simple example subcommand
    /// ```
    pub fn help_name_msg(&self, buf: &mut impl Write) -> std::io::Result<()> {
        let formatted_help = match self.help {
            Some(msg) => msg,
            None => HELP_DEFAULT,
        };

        writeln_term(format!("{} — {}", self.name, formatted_help), buf)
    }
}

/// Main cli structure, infomaton coming soon..
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

    /// Sets tabbing distance for current [CliMake], default is `2` spaces for
    /// tabs, chainable
    pub fn tabbing(&mut self, tab_size: impl Into<&'static str>) -> &mut Self {
        self.tabbing = tab_size.into();
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
    /// Usage:
    ///
    /// ```rust
    /// use std::io;
    /// use climake::CliMake;
    ///
    /// fn main() {
    ///     let cli = CliMake::new(
    ///         "My app", vec![], vec![], "A simple application", "0.1.0"
    ///     );
    ///
    ///     cli.header_msg(None, &mut io::stdout()).unwrap();
    /// }
    /// ```
    ///
    /// What this may display:
    ///
    /// ```none
    /// Usage: ./my-app [OPTIONS]
    ///
    ///   My app v0.1.0 — A simple application
    /// ```
    pub fn header_msg(
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

                writeln_term(
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
    /// Usage:
    ///
    /// ```rust
    /// use std::io;
    /// use climake::{CliMake, Argument, Input};
    ///
    /// fn main() {
    ///     let verbose = Argument::new(
    ///         "Toggles verbose mode",
    ///         vec!['v'],
    ///         vec!["verbose"],
    ///         Input::None
    ///     );
    ///
    ///     let cli = CliMake::new(
    ///         "My app", vec![&verbose], vec![], "A simple application", "0.1.0"
    ///     );
    ///
    ///     cli.help_msg(&mut io::stdout()).unwrap();
    /// }
    /// ```
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
    pub fn help_msg(&self, buf: &mut impl Write) -> std::io::Result<()> {
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
}

/// Writes a given buffer to terminal using [LineWriter] and splits every 80
/// characters, making it ideal for concise terminal displays for help messages
fn writeln_term(to_write: impl Into<String>, buf: &mut impl Write) -> std::io::Result<()> {
    let mut line_buf = LineWriter::new(buf);
    let newline_byte = "\n".as_bytes();

    for line in to_write.into().as_bytes().chunks(80 - CLI_TABBING.len()) {
        line_buf.write(&[CLI_TABBING.as_bytes(), line, newline_byte].concat())?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that the [Argument::new] method (creation of arguments) works correctly
    #[test]
    fn arg_new() {
        assert_eq!(
            Argument::new(None, vec!['a', 'b'], vec!["hi", "there"], Input::Text),
            Argument {
                calls: vec![
                    CallType::Short('a'),
                    CallType::Short('b'),
                    CallType::Long("hi".to_string()),
                    CallType::Long("there".to_string())
                ],
                help: None,
                input: Input::Text,
                required: false,
            }
        )
    }

    /// Checks that the [Argument::help_name_msg] method works correctly
    #[test]
    fn arg_name_help() -> std::io::Result<()> {
        let mut chk_vec: Vec<u8> = vec![];

        Argument::new(None, vec![], vec![], Input::None).help_name_msg(&mut chk_vec)?;
        assert_eq!(
            std::str::from_utf8(chk_vec.as_slice()).unwrap(),
            "  () — No help provided\n"
        );
        chk_vec = vec![];

        Argument::new("Some simple help", vec!['a'], vec!["long"], Input::Text)
            .help_name_msg(&mut chk_vec)?;
        assert_eq!(
            std::str::from_utf8(chk_vec.as_slice()).unwrap(),
            "  (-a, --long) [text] — Some simple help\n"
        );
        chk_vec = vec![];

        Argument::new(None, vec!['a'], vec![], Input::Text).help_name_msg(&mut chk_vec)?;
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

        let mut arg = Argument::new("Some argument", vec!['s'], vec![], Input::None);
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
        let arg = Argument::new("arg help", vec![], vec![], Input::None);

        cli.add_arg(&arg).add_arg(&arg);

        assert_eq!(cli.arguments, vec![&arg, &arg])
    }

    /// Checks that the [CliMake::add_args] method works correctly
    #[test]
    fn cli_add_args() {
        let mut cli = CliMake::new("example", vec![], vec![], "Add arg check", None);
        let arg = Argument::new("arg help", vec![], vec![], Input::None);

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
