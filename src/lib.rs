//! climake is a minimal-dependancies library for making simple arguments. This
//! libraries aim is not features but to provide a simple way to parse arguments
//! well enough with not much more processing used than the provided [std::env]
//! from the standard library.
//!
//! For more infomation, please see the [CLIMake] object and [Argument] to get
//! started parsing arguments using this library.

mod parse;
pub use parse::*;

/// The way the argument is called, can short or long. This enum is made to be
/// used in a [Vec] as then you may have multiple ways to call it.
pub enum CliCallType {
    /// Short call only, for example the `h` in `-hijk`.
    Short(char),

    /// Long call only, for example the `qwerty` in `--qwerty`.
    Long(String),
}

/// A single argument in a list of arguments to parse in [CliMake].
pub struct CliArgument {
    /// The way(s) in which you call this argument, used internally.
    pub calls: Vec<CliCallType>,

    /// Optional inner-command help.
    pub help: String,
}

impl CliArgument {
    /// Creates a new argument
    pub fn new(short_calls: Vec<char>, long_calls: Vec<String>, help: Option<String>) -> Self {
        let mut calls: Vec<CliCallType> = Vec::new();

        for short_call in short_calls {
            calls.push(CliCallType::Short(short_call));
        }

        for long_call in long_calls {
            calls.push(CliCallType::Long(long_call));
        }

        if help.is_some() {
            return CliArgument {
                calls: calls,
                help: help.unwrap(),
            };
        }

        CliArgument {
            calls: calls,
            help: String::from("No extra CLI help provided."),
        }
    }
}

/// Main holder structure of entire CliMake library.
pub struct CliMake {
    /// Arguments that this library parses.
    pub arguments: Vec<CliArgument>,

    /// Help message, optionally provided by user.
    pub help: String,
}

impl CliMake {
    /// Creates a new [CliMake] from arguments and optional help.
    pub fn new(arguments: Vec<CliArgument>, help: Option<String>) -> Self {
        if help.is_some() {
            return CliMake {
                arguments: arguments,
                help: help.unwrap(),
            };
        }

        CliMake {
            arguments: arguments,
            help: String::from("No extra argument help provided."),
        }
    }
}
