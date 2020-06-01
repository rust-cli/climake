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
//! fn other_arg_main(_args: Vec<String>) {
//!     println!("The normal --other or -o or -t argument.");
//! }
//!
//! fn main() {
//!     let qwerty_arg = CliArgument::new(
//!         vec!['q'],
//!         vec!["qwerty"],
//!         Some("Some useful help info."),
//!         Box::new(&qwerty_run_me), // this could be any closure/func with the arg being `Vec<String>`
//!     );
//!
//!     let other_arg = CliArgument::new(
//!         vec!['o', 't'],
//!         vec!["other"],
//!         None, // no help here!
//!         Box::new(&other_arg_main),
//!     );
//!
//!     let cli = CliMake::new(
//!         vec![qwerty_arg, other_arg],
//!         Some("This is some help info for this example CLI."),
//!     )
//!     .unwrap();
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
//! [dependancies]
//! climake = "1.0"
//! ```

#![allow(unused_assignments)] // RLS errors, shouldn't happen but does
#![doc(
    html_logo_url = "https://gitlab.com/Owez/climake/raw/master/logo.png",
    html_favicon_url = "https://gitlab.com/Owez/climake/raw/master/logo.png"
)]

use std::{env, process};

/// Error enum for climake when something goes wrong, ususally when adding/parsing
/// arguments.
#[derive(Debug)]
pub enum CliError {
    /// When an argument is duplicated. For example, if you had two arguments both
    /// with `-a` as a short call, this would raise as each arg call should be
    /// unique.
    ArgExists,
}

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
///     Box::new(&to_run_for_onetwo)
/// );
/// ```
pub struct CliArgument {
    /// Inner-command help for a specific argument. See [CliArgument::help_msg]
    /// for a better help representation.
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

    /// Gives in-depth details and running infomation compared to the plaintext
    /// [CliArgument::help_str], reccomended to use.
    ///
    /// ## Example results
    ///
    /// Both below are taken from the
    /// [dynamic args example](https://gitlab.com/Owez/climake/-/blob/master/examples/dynamic_args.rs):
    ///
    /// ```none
    /// Usage: ./dynamic_args [-q, -r, -s, --hi, --second] [CONTENT]
    ///
    /// About:
    ///   Simple help
    /// ```
    ///
    /// ```none
    /// Usage: ./dynamic_args [-a, -b, -c, --other, --thing] [CONTENT]
    ///
    /// About:
    ///   Other help
    /// ```
    pub fn help_msg(&self) -> String {
        let cur_exe = env::current_exe();
        let mut call_varients: Vec<String> = vec![];

        for call in self.calls.iter() {
            match call {
                CliCallType::Long(l) => call_varients.push(format!("--{}", l)),
                CliCallType::Short(s) => call_varients.push(format!("-{}", s)),
            }
        }

        format!(
            "Usage: ./{} [{}] [CONTENT]\n\nAbout:\n  {}",
            cur_exe.unwrap().file_stem().unwrap().to_str().unwrap(),
            call_varients.join(", "),
            self.help_str,
        )
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
///     vec![first_arg, other_arg],
///     Some("This is some help info for this example CLI.")
/// );
/// ```
pub struct CliMake {
    /// Arguments that this library parses.
    pub arguments: Vec<CliArgument>,

    /// Help message that user sees on a `--help` request or if nothing is
    /// passed/bad arguments passed.
    ///
    /// ## Example output
    ///
    /// The `A simple CLI.` in the following terminal output:
    ///
    /// ```none
    /// Usage: ./readme_showcase [OPTIONS]
    ///
    /// About:
    ///   This is some help info for this example CLI.
    ///
    /// Options:
    ///   [-q, --qwerty] - Some useful help info.
    ///   [-o, -t, --other] - No extra CLI help provided.
    /// ```
    pub help_str: &'static str,
}

impl CliMake {
    /// Creates a new [CliMake] from arguments and optional help.
    pub fn new(arguments: Vec<CliArgument>, help: Option<&'static str>) -> Result<Self, CliError> {
        let clean_help = match help {
            Some(h) => h,
            None => "No extra argument help provided.",
        };

        let mut cli = CliMake {
            arguments: vec![],
            help_str: clean_help,
        };

        for arg in arguments {
            cli.add_arg(arg)?; // prevent code dupe
        }

        Ok(cli)
    }

    /// Parses arguments from command line and automatically runs the closures
    /// optionally given for [CliArgument] or displays help infomation.
    pub fn parse(&self) {
        // TODO error message with help when an invalid arg is given
        let mut to_run: Option<&CliArgument> = None;
        let mut run_buffer: Vec<String> = Vec::new();

        let main_args = env::args();

        if main_args.len() == 1 {
            // show general help and exit with code 1
            eprintln!("{}", self.help_msg());
            process::exit(1);
        }

        for (arg_ind, arg) in main_args.enumerate() {
            if arg_ind == 0 {
                continue; // don't register first arg which gives system info
            } else if arg_ind == 1 && (arg == String::from("--help") || arg == "-h") {
                // show general help and exit with code 0
                println!("{}", self.help_msg());
                process::exit(0);
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
                                if run_buffer.len() == 0 && arg == String::from("--help") {
                                    // show arg-specific help and exit with code 0
                                    println!("{}", r.help_msg());
                                    process::exit(0);
                                }

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
    pub fn add_arg(&mut self, argument: CliArgument) -> Result<(), CliError> {
        for call in argument.calls.iter() {
            let possible_dupe = self.search_arg(call.clone());

            if possible_dupe.is_some() {
                return Err(CliError::ArgExists);
            }
        }

        self.arguments.push(argument);

        Ok(())
    }

    /// Returns parsed help message as a [String].
    pub fn help_msg(&self) -> String {
        let cur_exe = env::current_exe();

        let mut arg_help: Vec<String> = vec![];

        for arg in self.arguments.iter() {
            let mut arg_vec = Vec::new();

            for call in arg.calls.iter() {
                match call {
                    CliCallType::Long(l) => arg_vec.push(format!("--{}", l)),
                    CliCallType::Short(s) => arg_vec.push(format!("-{}", s)),
                }
            }

            arg_help.push(format!("  [{}] - {}", arg_vec.join(", "), arg.help_str));
        }

        format!(
            "Usage: ./{} [OPTIONS]\n\nAbout:\n  {}\n\nOptions:\n{}",
            cur_exe.unwrap().file_stem().unwrap().to_str().unwrap(),
            self.help_str,
            arg_help.join("\n")
        )
    }

    /// Searches for an argument in self using a [CliCallType] as an easy way to
    /// search both short and long args.
    fn search_arg(&self, query: CliCallType) -> Option<&CliArgument> {
        // TODO: make this into `Result<&CliArgument, CliError>` with a new CliError
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
        let cli = CliMake::new(cli_args, Some("A simple CLI.")).unwrap();

        cli.help_msg();
    }
}
