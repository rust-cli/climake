//! The simple, dependency-less cli library ✨
//!
//! ## Example 📚
//!
//! ```should_panic
//! use climake::{Argument, CLIMake, DataType};
//!
//! fn main() {
//!     let args = &[
//!         Argument::new(
//!             &['o'],
//!             &["output", "out"],
//!             Some("Example output arg"),
//!             DataType::File,
//!         ).unwrap(),
//!         Argument::new(
//!             &['a', 'b', 'c'],
//!             &[],
//!             Some("Alphabet!"),
//!             DataType::None,
//!         ).unwrap(),
//!     ];
//!
//!     let cli = CLIMake::new(args, Some("A showcase CLI to demonstrate climake"), None).unwrap();
//!
//!     println!("Args used:\n{:#?}", cli.parse());
//! }
//! ```
//!
//! ## Installation 🚀
//!
//! Simply add the following to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! climake = "2.1"
//! ```

#![doc(
    html_logo_url = "https://github.com/rust-cli/climake/raw/master/logo.png",
    html_favicon_url = "https://github.com/rust-cli/climake/raw/master/logo.png"
)]

use std::{env, fmt, path::PathBuf, process};

/// The primary error enum for climake, used when an error is encountered to use
/// downstream
#[derive(Debug, PartialEq, Clone)]
pub enum CLIError {
    /// This raises when there are no calls defined in a given [Argument] when
    /// creating using [Argument::new]
    NoCalls,

    /// An argument's call was already added to the CLI. This means a [CallType]
    /// has been exactly duplicated for an [Argument]
    ArgExists,

    /// When a referenced [CallType] could not be found.
    ///
    /// This is used interally inside of climake, if this is produced elsewhere
    /// **an issue should be filed**.
    ArgNotFound,
}

/// The type of data an argument accepts. The enum that hands the user's inputs
/// to you is [PassedData], extending from [UsedArg]
#[derive(Debug, PartialEq, Clone)]
pub enum DataType {
    /// Doesn't accept any data and throws an error if data is passed,
    /// will return nothing
    None,

    /// Plaintext (typically used), will return a [String]
    Text,

    /// A file or directory, will return a [PathBuf]
    File,
}

impl fmt::Display for DataType {
    /// String representation of [DataType], used for downstream help messages
    /// for individual args, see [Argument::pretty_help]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataType::None => write!(f, ""),
            DataType::Text => write!(f, " [TEXT]"),
            DataType::File => write!(f, " [FILE]"),
        }
    }
}

/// Data collected from user input based upon wanted data from [PassedData]
#[derive(Debug, PartialEq, Clone)]
pub enum PassedData {
    /// No data given. This is used when no data was given for any [DataType],
    /// not just [DataType::None]
    None,

    /// Successfully got some text from user, will be returned if [DataType::Text]
    /// is set for an argument
    Text(Vec<String>),

    /// Successfully got a file or directory from user, will be returned if
    /// [DataType::File] is set for an argument
    File(Vec<PathBuf>),
}

/// The ways users can call a given [Argument]
#[derive(Debug, PartialEq, Clone)]
pub enum CallType<'a> {
    /// Short, 1 character long `-t`-type calls
    Short(char),

    /// Long `--test`-type calls
    Long(&'a str),
}

/// An allowed argument for a new CLI
#[derive(Debug, PartialEq, Clone)]
pub struct Argument<'a> {
    /// Ways users can call an argument. If this is a length of 0 at compile-time,
    /// climake will raise [CLIError::NoCalls]
    pub calls: Vec<CallType<'a>>,

    /// Help message if any (will display "no help given" if nothing is shown here)
    pub help: Option<&'a str>,

    /// Data this argument accepts
    pub datatype: DataType,
}

impl<'a> Argument<'a> {
    /// Shortcut method for creating an [Argument]
    pub fn new(
        short_calls: &[char],
        long_calls: &[&'a str],
        help: Option<&'a str>,
        datatype: DataType,
    ) -> Result<Self, CLIError> {
        if short_calls.len() + long_calls.len() == 0 {
            return Err(CLIError::NoCalls);
        }

        let mut calls: Vec<CallType> = Vec::new();

        for sc in short_calls {
            calls.push(CallType::Short(*sc));
        }

        for lc in long_calls {
            calls.push(CallType::Long(lc))
        }

        Ok(Self {
            calls,
            help,
            datatype,
        })
    }

    /// Creates a pretty, formatted help string for use in help messages by default
    pub fn pretty_help(&self) -> String {
        let mut call_varients: Vec<String> = Vec::new();

        for call in self.calls.iter() {
            match call {
                CallType::Long(l) => call_varients.push(format!("--{}", l)),
                CallType::Short(s) => call_varients.push(format!("-{}", s)),
            }
        }

        let formatted_help = match self.help {
            Some(msg) => msg,
            None => "No help message provided",
        };

        format!(
            "\n  ({}){}: {}",
            call_varients.join(", "),
            self.datatype,
            formatted_help,
        )
    }
}

/// Given for when a user used a valid [Argument] and any data given
/// alongside it
#[derive(Debug, PartialEq, Clone)]
pub struct UsedArg<'a> {
    /// Argument used
    pub argument: Argument<'a>,

    /// Data passed by user. See [PassedData]'s documentation for more info
    pub passed_data: PassedData,
}

impl<'a> UsedArg<'a> {
    /// Private shortcut creation method for [UsedArg] to be used inside of parsing
    fn new(arg: Argument<'a>, raw_data: Vec<String>) -> Self {
        match arg.datatype {
            DataType::None => Self {
                argument: arg,
                passed_data: PassedData::None,
            },
            DataType::Text => Self {
                argument: arg,
                passed_data: PassedData::Text(raw_data),
            },
            DataType::File => Self {
                argument: arg,
                passed_data: PassedData::File(
                    raw_data
                        .iter()
                        .map(|x| PathBuf::from(x))
                        .collect::<Vec<PathBuf>>(),
                ),
            },
        }
    }
}

/// Main holder structure of entire climake library, used to create new CLIs.
///
/// It is reccomended this be called something simple like `cli` for ease of use
/// as this is the most used part of climake.
///
/// ## Examples
///
/// ```should_panic
/// use climake::{Argument, CLIMake, DataType};
///
/// fn main() {
///     let args = &[
///         Argument::new(
///             &['o'],
///             &["output", "out"],
///             Some("Example output arg"),
///             DataType::File,
///         ).unwrap(),
///         Argument::new(
///             &['a', 'b', 'c'],
///             &[],
///             Some("Alphabet!"),
///             DataType::None,
///         ).unwrap(),
///     ];
///
///     let cli = CLIMake::new(args, Some("A showcase CLI to demonstrate climake"), None).unwrap();
///
///     println!("Args used:\n{:#?}", cli.parse());
/// }
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct CLIMake<'cli, 'a> {
    /// Arguments to use for CLI instance
    pub args: &'cli [Argument<'a>],

    /// Optional description of CLI
    pub description: Option<&'static str>,

    /// Optional version of CLI/program
    ///
    /// ## Finding your crate's version
    ///
    /// You can use the following snippet to find out your crates version:
    ///
    /// ```rust
    /// #[macro_export]
    /// macro_rules! crate_version {
    ///     () => {
    ///         format!("{}.{}.{}{}",
    ///         env!("CARGO_PKG_VERSION_MAJOR"),
    ///         env!("CARGO_PKG_VERSION_MINOR"),
    ///         env!("CARGO_PKG_VERSION_PATCH"),
    ///         option_env!("CARGO_PKG_VERSION_PRE").unwrap_or(""))
    ///     }
    /// }
    /// ```
    ///
    /// *Taken from [clap's `crate_version`](https://docs.rs/clap/2.33.3/clap/macro.crate_version.html)*
    pub version: Option<String>,
}

impl<'cli, 'a> CLIMake<'cli, 'a> {
    /// Shortcut to making a [CLIMake] structure, the main entrypoint into
    /// building a CLI with climake
    pub fn new(
        args: &'cli [Argument<'a>],
        description: Option<&'static str>,
        version: Option<String>,
    ) -> Result<Self, CLIError> {
        if slice_has_dup(args) {
            return Err(CLIError::ArgExists);
        }

        Ok(Self {
            args,
            description,
            version,
        })
    }

    /// Header message to be used above help or errors to show the CLI has been
    /// at least successfully initiated and to show basic info about the program
    pub fn header_msg(&self) -> String {
        let cur_exe = env::current_exe();

        let topline = format!(
            "Usage: ./{} [OPTIONS]",
            cur_exe.unwrap().file_stem().unwrap().to_str().unwrap()
        );

        match self.description {
            Some(description) => match self.version.clone() {
                Some(version) => format!("{}\n\n  v{} - {}", topline, version, description),
                None => format!("{}\n\n  {}", topline, description),
            },
            None => format!("{}\n\n", topline),
        }
    }

    /// Overall help for built CLI, displays header and every arg
    /// using [Argument::pretty_help]
    pub fn help_msg(&self) -> String {
        let mut output = format!("{}\n\nOptions:", self.header_msg());

        for arg in self.args {
            output += &arg.pretty_help();
        }

        output
    }

    /// Shortcut to providing help message and exiting with error code 1
    fn error_help(&self, msg: Option<&str>) -> ! {
        match msg {
            Some(m) => eprintln!("Error: {}\n{}", m, self.help_msg()),
            None => eprintln!("{}", self.help_msg()),
        };
        process::exit(1);
    }

    /// Produces a [Argument::pretty_help] with CLI's header to be used for
    /// arg-specific help messages
    pub fn specific_help(&self, arg: &Argument) -> String {
        format!("{}\n\nArg help:{}", self.header_msg(), arg.pretty_help())
    }

    /// Searches for an argument in self using a [CallType] as an easy way to
    /// search both short and long args.
    fn search_arg(&self, query: CallType) -> Result<&Argument, CLIError> {
        for arg in self.args.iter() {
            for call in arg.calls.iter() {
                if call == &query {
                    return Ok(&arg);
                }
            }
        }

        Err(CLIError::ArgNotFound)
    }

    /// Parses arguments and returns all [UsedArg]s
    pub fn parse(&self) -> Vec<UsedArg> {
        let mut args_output: Vec<UsedArg> = Vec::new();

        let mut tmp_arg_data: Vec<String> = Vec::new();
        let mut tmp_arg: Option<&Argument> = None;

        let passed_args = env::args();

        if passed_args.len() == 1 {
            self.error_help(Some("No arguments given"));
        }

        for (arg_ind, arg) in passed_args.enumerate() {
            // each full arg

            if arg_ind == 0 {
                continue; // don't register sysinfo arg
            } else if arg_ind == 1 && (arg == "--help" || arg == "-h") {
                // asked for help, return help with code 0
                println!("{}", self.help_msg());
                process::exit(0);
            }

            let mut arg_possible = false; // flips to detect - or -- args

            for (char_ind, character) in arg.chars().enumerate() {
                // each letter of arg

                if character == '-' {
                    if char_ind == 0 {
                        // possible short or long arg
                        arg_possible = true;
                        continue;
                    } else if char_ind == 1 {
                        // long arg, add to

                        match tmp_arg {
                            Some(ta) => {
                                // show arg-specific help and exit with code 0
                                if tmp_arg_data.len() == 0 && arg == String::from("--help") {
                                    println!("{}", self.specific_help(ta));
                                    process::exit(0);
                                }
                            }
                            None => (),
                        };

                        tmp_arg = match self.search_arg(CallType::Long(&arg[2..])) {
                            Ok(x) => Some(x),
                            Err(_) => self.error_help(Some("Unknown long arg")),
                        };

                        break;
                    }
                }

                if arg_possible {
                    match tmp_arg {
                        Some(a) => {
                            // add arg to output then drain data

                            args_output.push(UsedArg::new(a.clone(), tmp_arg_data.clone()));
                            tmp_arg_data.drain(..);
                        }
                        None => (),
                    };

                    // possible short arg, just search and if it isn't, leave it the same
                    tmp_arg = match self.search_arg(CallType::Short(character)) {
                        Ok(x) => Some(x),
                        Err(_) => self.error_help(Some("Unknown short arg")),
                    };
                } else {
                    tmp_arg_data.push(arg.clone());
                    break;
                }
            }

            if arg_ind + 1 == env::args().len() {
                match tmp_arg {
                    Some(a) => {
                        args_output.push(UsedArg::new(a.clone(), tmp_arg_data));
                        break; // used so no cloning of `tmp_arg_data`
                    }
                    None => (),
                }
            }
        }

        args_output
    }
}

/// Checks for duplications, used for [CLIMake::new] arguments
fn slice_has_dup<T: PartialEq>(slice: &[T]) -> bool {
    for i in 1..slice.len() {
        if slice[i..].contains(&slice[i - 1]) {
            return true;
        }
    }
    false
}
