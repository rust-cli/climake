//! The simple, dependancy-less cli framework ✨
//!
//! ## Example 📚
//!
//! ```rust
//! use climake::*;
//!
//! /// This will be ran when the -q (or --qwerty) argument is ran. args are the
//! /// arguments passed.
//! fn qwerty_run_me(args: Vec<String>) {
//!     println!(
//!         "The -q (or --qwerty) argument was ran! Here are the arguments passed: {:?}.",
//!         args
//!     );
//! }
//!
//! fn other_arg_main(args: Vec<String>) {
//!     println!("The normal --other or -o or -t argument.");
//! }
//!
//! fn main() {
//!     let qwerty_arg = CliArgument::new(
//!         vec!['q'],
//!         vec!["qwerty"],
//!         Some("Some useful help info."),
//!         Box::new(&qwerty_run_me) // this could be any closure/func with the arg being `Vec<String>`
//!     );
//!
//!     let other_arg = CliArgument::new(
//!         vec!['o', 't'],
//!         vec!["other"],
//!         None, // no help here!
//!         Box::new(&other_arg_main)
//!     );
//!
//!     let cli = CliMake::new(
//!         "Example CLI",
//!         vec![qwerty_arg, other_arg],
//!         Some("This is some help info for this example CLI.")
//!     );
//!
//!     cli.parse() // runs all given parts like qwerty_run_me if called
//! }
//! ```
//!
//! ## Installation 🚀
//!
//! Simply add the following to your `Cargo.toml` file:
//!
//! ```toml
//! climake = "1.0"
//! ```

#![allow(unused_assignments)] // strange rls errors for something that doesn't exist

use std::env;

/// The way the argument is called, can short or long. This enum is made to be
/// used in a [Vec] as then you may have multiple ways to call it.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum CliCallType {
    /// Short call only, for example the `h` in `-hijk`.
    Short(char),

    /// Long call only, for example the `qwerty` in `--qwerty`.
    ///
    /// Using [String] here as its much easier than trying to do &[str] lifetimes.
    Long(String),
}

/// A single argument in a list of arguments to parse later in [CliMake::parse].
///
/// ## Example inititation
///
/// ```ignore
/// let arg_onetwo = CliArgument.new(
///     vec!['o', 't'],
///     vec!["onetwo"],
///     Some("This is some detailed help for onetwo"),
///     to_run_for_onetwo
/// );
/// ```
pub struct CliArgument {
    /// Inner-command help, similar to [CliMake::help_str] but only for help
    /// regarding this argument.
    pub help_str: &'static str,

    /// The way(s) in which you call this argument, used internally.
    calls: Vec<CliCallType>,

    /// What to run if the argument is called. This will always pass an argument
    /// to the runnable function which is a [Vec]<[String]> due to potential
    /// arguments passed, used internally.
    run: Box<dyn Fn(Vec<String>)>,
}

impl CliArgument {
    /// Creates a new argument in a simplistic manner.
    pub fn new(
        short_calls: Vec<char>,
        long_calls: Vec<&'static str>,
        help: Option<&'static str>,
        run: Box<dyn Fn(Vec<String>)>,
    ) -> Self {
        let mut calls: Vec<CliCallType> = Vec::new();

        for short_call in short_calls {
            calls.push(CliCallType::Short(short_call));
        }

        for long_call in long_calls {
            calls.push(CliCallType::Long(String::from(long_call)));
        }

        if help.is_some() {
            return CliArgument {
                calls: calls,
                help_str: help.unwrap(),
                run: run,
            };
        }

        CliArgument {
            calls: calls,
            help_str: "No extra CLI help provided.",
            run: run,
        }
    }
}

/// Main holder structure of entire climake library, used to create new CLIs.
///
/// It is reccomended this be called something simple like `cli` for ease of use
/// as this is the most used part of climake.
///
/// ## Example initiation
///
/// ```ignore
/// let cli = CliMake.new(
///     "Example CLI"
///     vec![first_arg, other_arg],
///     Some("This is some help info for this example CLI.")
/// );
/// ```
pub struct CliMake {
    /// Arguments that this library parses.
    pub arguments: Vec<CliArgument>,

    /// Name of CLI displayed on help page.
    pub name: &'static str,

    /// Help message that user sees on a `--help` request or if nothing is
    /// passed/bad arguments passed.
    ///
    /// ## Example output
    ///
    /// The `A simple CLI.` in the following terminal output:
    ///
    /// ```none
    /// Usage: ./climake [OPTIONS]
    ///
    /// A simple CLI.
    ///
    /// Options:
    /// -q, -r, -s, --hi, --second | Simple help
    /// -a, -b, -c, --other, --thing | Other help
    ///
    /// ```
    pub help_str: &'static str,
}

impl CliMake {
    /// Creates a new [CliMake] from arguments and optional help.
    pub fn new(
        name: &'static str,
        arguments: Vec<CliArgument>,
        help: Option<&'static str>,
    ) -> Self {
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
            help_str: "No extra argument help provided.",
        }
    }

    /// Parses arguments from command line and automatically runs the closures
    /// optionally given for [CliArgument] or displays help infomation.
    pub fn parse(&self) {
        let mut to_run: Option<&CliArgument> = None;
        let mut run_buffer: Vec<String> = Vec::new();

        for (arg_ind, arg) in env::args().enumerate() {
            if arg_ind == 0 {
                continue; // don't register first arg which gives system info
            }

            let mut arg_possible = false;

            for (ind_char, character) in arg.chars().enumerate() {
                if character == '-' {
                    if ind_char == 0 {
                        // possible short arg
                        arg_possible = true;
                        continue;
                    } else if ind_char == 1 {
                        match to_run {
                            Some(r) => {
                                // run then destroy
                                (r.run)(run_buffer.clone());
                                to_run = None;
                                run_buffer.drain(..);
                            }
                            None => (),
                        }

                        // long arg
                        let clean_arg = String::from(&arg[2..]);
                        to_run = self.search_arg(CliCallType::Long(clean_arg));

                        break;
                    }
                }

                if arg_possible {
                    match to_run {
                        Some(r) => {
                            // run then destroy
                            (r.run)(run_buffer.clone());

                            to_run = None;
                            run_buffer.drain(..);
                        }
                        None => (),
                    }

                    // short arg
                    to_run = self.search_arg(CliCallType::Short(character));
                } else {
                    // content of other arg
                    run_buffer.push(arg);
                    break;
                }
            }

            if arg_ind + 1 == env::args().len() {
                // last arg, call any remaining to_run + run_buffer
                match to_run {
                    Some(r) => (r.run)(run_buffer.clone()),
                    None => (),
                }
            }
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

        for arg in self.arguments.iter() {
            let mut arg_vec = Vec::new();

            for call in arg.calls.iter() {
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

    /// Searches for an argument in self using a [CliCallType] as an easy way to
    /// search both short and long args.
    fn search_arg(&self, query: CliCallType) -> Option<&CliArgument> {
        for argument in self.arguments.iter() {
            for call in argument.calls.iter() {
                if call == &query {
                    return Some(&argument);
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Ensures help message displays without errors.
    #[test]
    fn help_msg() {
        /// Internal func to run for args
        fn test_func(args: Vec<String>) {
            println!("It works! Found args: {:?}", args);
        }

        let cli_args = vec![
            CliArgument::new(
                vec!['q', 'r', 's'],
                vec!["hi", "second"],
                Some("Simple help"),
                Box::new(test_func),
            ),
            CliArgument::new(
                vec!['a', 'b', 'c'],
                vec!["other", "thing"],
                Some("Other help"),
                Box::new(test_func),
            ),
        ];
        let cli = CliMake::new("Test CLI", cli_args, Some("A simple CLI."));

        cli.help_msg();
    }
}

// fn main() {
//     /// Internal func to run for args
//     fn test_func(args: Vec<String>) {
//         println!("It works! Found args: {:?}", args);
//     }

//     let cli_args = vec![
//         CliArgument::new(
//             vec!['q', 'r', 's'],
//             vec!["hi", "second"],
//             Some("Simple help"),
//             Box::new(test_func),
//         ),
//         CliArgument::new(
//             vec!['a', 'b', 'c'],
//             vec!["other", "thing"],
//             Some("Other help"),
//             Box::new(test_func),
//         ),
//     ];
//     let cli = CliMake::new("Test CLI", cli_args, Some("A simple CLI."));

//     println!("{}", cli.help_msg());
// }
