//! The simple, dependency-less cli library ✨
//!
//! ## Example 📚
//!
//! Rewrite example coming soon!
//!
//! ## Installation 🚀
//!
//! Simply add the following to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! climake = "2.0"
//! ```

#![doc(
    html_logo_url = "https://github.com/Owez/climake/raw/master/logo.png",
    html_favicon_url = "https://github.com/Owez/climake/raw/master/logo.png"
)]

use std::{env, fmt, path::PathBuf, process};

/// The primary error enum for climake, used when an error is encountered to use
/// downstream
#[derive(Debug, PartialEq, Clone)]
pub enum CLIError {
    /// This raises when calls defined in [Argument] at compile-time
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

    /// Plaintext (typically used), will return a [String]. Errors if no data is passed
    Text,

    /// A file or directory, will return a [PathBuf]. Errors if no data is passed
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
pub enum CallType {
    /// Short, 1 character long `-t`-type calls
    Short(char),

    /// Long `--test`-type calls
    Long(String),
}

/// An allowed argument for a new CLI
#[derive(Debug, PartialEq, Clone)]
pub struct Argument {
    /// Ways users can call an argument. If this is a length of 0 at compile-time,
    /// climake will raise [CLIError::NoCalls]
    pub calls: Vec<CallType>,

    /// Help message if any (will display "no help given" if nothing is shown here)
    pub help: Option<&'static str>,

    /// Data this argument accepts
    pub datatype: DataType,
}

impl Argument {
    /// Shortcut method for creating an [Argument]
    pub fn new(
        short_calls: Vec<char>,
        long_calls: Vec<String>,
        help: Option<&'static str>,
        datatype: DataType,
    ) -> Self {
        let mut calls: Vec<CallType> = Vec::new();

        for sc in short_calls {
            calls.push(CallType::Short(sc));
        }

        for lc in long_calls {
            calls.push(CallType::Long(lc))
        }

        Self {
            calls,
            help,
            datatype,
        }
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
pub struct UsedArg {
    /// Argument used
    pub argument: Argument,

    /// Data passed by user. See [PassedData]'s documentation for more info
    pub passed_data: PassedData,
}

impl UsedArg {
    /// Private shortcut creation method for [UsedArg] to be used inside of parsing
    fn new(arg: Argument, raw_data: Vec<String>) -> Self {
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
///     let args = vec![
///         Argument::new(
///             vec!['o'],
///             vec![String::from("output"), String::from("out")],
///             Some("Example output arg"),
///             DataType::File,
///         ),
///         Argument::new(
///             vec!['a', 'b', 'c'],
///             vec![],
///             Some("Alphabet!"),
///             DataType::None,
///         ),
///     ];
///
///     let cli = CLIMake::new(args, Some("A showcase CLI to demonstrate climake"), None);
///
///     println!("Args used:\n{:#?}", cli.parse());
/// }
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct CLIMake {
    /// Arguments to use for CLI instance
    pub args: Vec<Argument>,

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

impl CLIMake {
    /// Shortcut to making a [CLIMake] structure, the main entrypoint into
    /// building a CLI with climake
    pub fn new(
        args: Vec<Argument>,
        description: Option<&'static str>,
        version: Option<String>,
    ) -> Self {
        Self {
            args,
            description,
            version,
        }
    }

    /// Header message to be used above help or errors to show the CLI has been
    /// at least successfully initiated and to show basic info about the program
    fn header_msg(&self) -> String {
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

    /// Overall help for built CLI, displays header and each args [Argument::pretty_help]
    fn help_msg(&self) -> String {
        let mut output = format!("{}\n\nOptions:", self.header_msg());

        for arg in &self.args {
            output += &arg.pretty_help();
        }

        output
    }

    /// Shortcut to providing help message and exiting with error code 1
    fn error_help(&self) -> ! {
        eprintln!("{}", self.help_msg());
        process::exit(1);
    }

    /// Produces a [Argument::pretty_help] with CLI's header to be used for
    /// arg-specific help messages
    fn specific_help(&self, arg: &Argument) -> String {
        format!("{}Arg help:{}", self.header_msg(), arg.pretty_help())
    }

    /// Adds new argument to instanced cli
    pub fn add_arg(&mut self, arg: Argument) -> Result<(), CLIError> {
        for call in arg.calls.iter() {
            match self.search_arg(call.clone()) {
                Ok(_) => return Err(CLIError::ArgExists),
                Err(_) => (),
            }; // searches for dupes, essentially turns [CLIMake::search_arg] around
        }

        self.args.push(arg);

        Ok(())
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
            self.error_help();
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

                        let stripped_arg = String::from(&arg[2..]);

                        tmp_arg = match self.search_arg(CallType::Long(stripped_arg)) {
                            Ok(x) => Some(x),
                            Err(_) => self.error_help(),
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
                        Err(_) => None,
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Internal helper for tests that removes first `lines` lines from given
    /// [String] `input`
    fn remove_lines(input: String, lines: usize) -> String {
        input.split("\n").collect::<Vec<&str>>()[lines..].join("\n")
    }

    /// Ensures header message displays without errors.
    ///
    /// *This is not checked with any [assert_eq] as header messages change with
    /// binary name*
    #[test]
    fn check_header() {
        let cli = CLIMake::new(Vec::new(), Some("A simple CLI."), None);

        cli.header_msg();
    }

    /// Tests individual arg's `pretty_help` message
    #[test]
    fn check_arg_help() {
        let arg_1 = Argument::new(
            vec!['q', 'r', 's'],
            vec![String::from("hi"), String::from("second")],
            Some("Simple help"),
            DataType::None,
        );
        let arg_2 = Argument::new(
            vec!['a', 'b', 'c'],
            vec![String::from("other"), String::from("thing")],
            Some("Other help"),
            DataType::None,
        );
        let arg_3 = Argument::new(vec!['o'], vec![], None, DataType::None);

        assert_eq!(
            arg_1.pretty_help(),
            "\n  (-q, -r, -s, --hi, --second): Simple help"
        );
        assert_eq!(
            arg_2.pretty_help(),
            "\n  (-a, -b, -c, --other, --thing): Other help"
        );
        assert_eq!(arg_3.pretty_help(), "\n  (-o): No help message provided");
    }

    /// Checks that the cli can parse a full help message compared to a correct
    /// help message
    #[test]
    fn cli_full_help() {
        const TRUE_HELP: &str = "  A simple debug cli\n\nOptions:\n  (-q, -r, --hi): Simple help\n  (-o, --2nd, --arg): A simple second arg";

        let cli_args = vec![
            Argument::new(
                vec!['q', 'r'],
                vec![String::from("hi")],
                Some("Simple help"),
                DataType::None,
            ),
            Argument::new(
                vec!['o'],
                vec![String::from("2nd"), String::from("arg")],
                Some("A simple second arg"),
                DataType::None,
            ),
        ];
        let cli = CLIMake::new(cli_args, Some("A simple debug cli"), None);

        assert_eq!(remove_lines(cli.help_msg(), 2), TRUE_HELP);
    }

    /// Checks that args return proper specific help messages
    #[test]
    fn specific_arg_help() {
        const TRUE_HELP: &str = "Arg help:\n  (-t): Specific help";

        let arg = Argument::new(vec!['t'], vec![], Some("Specific help"), DataType::None);
        let cli = CLIMake::new(vec![arg.clone()], None, None);

        assert_eq!(remove_lines(cli.specific_help(&arg), 2), TRUE_HELP);
    }
}
