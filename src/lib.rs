//! climake is a minimal-dependancies library for making simple arguments. This
//! libraries aim is not features but to provide a simple way to parse arguments
//! well enough with not much more processing used than the provided [std::env]
//! from the standard library.
//!
//! For more infomation, please see the [CLIMake] object and [Argument] to get
//! started parsing arguments using this library.

use std::env;

/// The way the argument is called, can short or long. This enum is made to be
/// used in a [Vec] as then you may have multiple ways to call it.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum CliCallType {
    /// Short call only, for example the `h` in `-hijk`.
    Short(char),

    /// Long call only, for example the `qwerty` in `--qwerty`.
    Long(String),
}

/// A single argument in a list of arguments to parse in [CliMake].
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CliArgument {
    /// The way(s) in which you call this argument, used internally.
    pub calls: Vec<CliCallType>,

    /// Optional inner-command help.
    pub help_str: String,
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
                help_str: help.unwrap(),
            };
        }

        CliArgument {
            calls: calls,
            help_str: String::from("No extra CLI help provided."),
        }
    }
}

/// Main holder structure of entire CliMake library.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CliMake {
    /// Arguments that this library parses.
    pub arguments: Vec<CliArgument>,

    /// Name of CLI displayed on help page.
    pub name: String,

    /// Help message, optionally provided by user.
    pub help_str: String,
}

impl CliMake {
    /// Creates a new [CliMake] from arguments and optional help.
    pub fn new(arguments: Vec<CliArgument>, name: String, help: Option<String>) -> Self {
        if help.is_some() {
            return CliMake {
                arguments: arguments,
                name: name,
                help_str: help.unwrap(),
            };
        }

        CliMake {
            arguments: arguments,
            name: name,
            help_str: String::from("No extra argument help provided."),
        }
    }

    /// Adds new argument to [CliMake]
    pub fn add_arg(&mut self, argument: CliArgument) {
        self.arguments.push(argument);
    }

    /// Returns parsed help message as a [String].
    pub fn help_msg(&self) -> String {
        let cur_exe = env::current_exe();

        let mut arg_help = String::new();

        for arg in self.arguments.clone() {
            let mut arg_vec = Vec::new();

            for call in arg.calls {
                match call {
                    CliCallType::Long(l) => arg_vec.push(format!("--{}", l)),
                    CliCallType::Short(s) => arg_vec.push(format!("-{}", s)),
                }
            }

            arg_help.push_str(&format!("{} | {}\n", arg_vec.join(", "), arg.help_str));
        }

        format!(
            "Usage: ./{} [OPTIONS]\n\n  {}\n\nOptions:\n{}",
            cur_exe.unwrap().file_stem().unwrap().to_str().unwrap(),
            self.help_str,
            arg_help
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Ensures help message displays without errors.
    #[test]
    fn help_msg() {
        let cli_args = vec![
            CliArgument::new(
                vec!['q', 'r', 's'],
                vec![String::from("hi"), String::from("second")],
                None,
            ),
            CliArgument::new(
                vec!['a', 'b', 'c'],
                vec![String::from("other"), String::from("thing")],
                None,
            ),
        ];
        let cli = CliMake::new(
            cli_args,
            String::from("Test CLI"),
            Some(String::from("A simple CLI.")),
        );

        cli.help_msg();
    }
}
