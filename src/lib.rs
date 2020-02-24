//! climake is a minimal-dependancies library for making simple arguments. This
//! libraries aim is not features but to provide a simple way to parse arguments
//! well enough with not much more processing used than the provided [std::env]
//! from the standard library.
//!
//! For more infomation, please see the [CLIMake] object and [Argument] to get
//! started parsing arguments using this library.

use std::env;

/// Primary error enum for exceptions related to methods inside of the climake
/// module.
#[derive(Debug)]
pub enum CLIMakeError {
    /// When attempting to parse arguments (usually inside of [CLIMake::parse_args])
    /// the user did not enter any valid arguments. This will not run if a
    /// [CLIMake::none_run] is given.
    NoArgumentsPassed,
}

/// Main structure for climake, the [CLIMake] object. An instance of this is
/// made with the standard rust structure initialsation and further arguments
/// can easily be added using [CLIMake::add_existing_arg].
pub struct CLIMake {
    /// Name of overall CLI
    pub name: String,

    /// Description of CLI (if any)
    pub description: Option<String>,

    /// Arguments included
    pub args: Vec<Argument>,

    /// An optional run parameter if no arguments are passed. If this is
    /// [Option::None] and no arguments are passed, climake will default to
    /// [CLIMakeError::NoArgumentsPassed].
    pub none_run: Option<Box<dyn Fn()>>,
}

impl CLIMake {
    /// Parses arguments given from the avalible [CLIMake::args] and runs the
    /// corrosponding tasks in order.
    ///
    /// **NOTE: This function will eventually close the program running using
    /// [std::process].**
    pub fn parse_args(&self) {
        // below are passed in arguments and a closure to search through vector above it
        let passed_args: Vec<String> = env::args().collect();
        let check_args = |query: String| passed_args.iter().position(|a| a == &query).is_some();

        if passed_args.len() == 1 {
            match &self.none_run {
                Some(to_run) => {
                    (to_run)();
                    std::process::exit(0); // exited successfully
                }
                None => {
                    println!("{}No arguments passed!", self.header_text());
                    std::process::exit(1); // exited with error
                }
            }
        }

        for arg in self.args.iter() {
            let short_call_pass = check_args(String::clone(&arg.short_call));
            let standalone_call_pass = match &arg.standalone_call {
                Some(x) => check_args(x.clone()),
                None => false,
            };

            if short_call_pass || standalone_call_pass {
                (arg.run)();
            }
        }

        std::process::exit(0); // exited successfully
    }

    /// Adds a new argument to parser.
    pub fn add_existing_arg(&mut self, new_arg: Argument) {
        self.args.push(new_arg)
    }

    /// Displays help message in `stdout` using added arguments.
    pub fn help_msg(&self) {
        let header_text = self.header_text();
        let mut generated_help = format!("{}Options:", header_text);

        if self.args.len() == 0 {
            return println!("{}No arguments made!", header_text);
        }

        for arg in self.args.iter() {
            let ensured_arg_help = match &arg.help {
                Some(help) => String::clone(help),
                None => String::from("Help not provided."),
            };

            let info_help = match &arg.standalone_call {
                Some(standalone_call) => format!(
                    "  -{} / -{} ({})",
                    arg.short_call, standalone_call, ensured_arg_help
                ),
                None => format!("  -{} ({})", arg.short_call, ensured_arg_help),
            };

            generated_help.push_str(&info_help);
        }

        println!("{}", generated_help);
    }

    /// Returns nicely formatted header text that is used for each stdout pass
    /// involving [CLIMaker].
    fn header_text(&self) -> String {
        let exe_name = env::current_exe().unwrap();
        let usage_info = format!(
            "Usage: ./{} [OPTIONS]",
            exe_name.file_name().unwrap().to_str().unwrap() // Thanks rust
        );

        if self.description.is_some() {
            format!(
                "{}\n\n\t{}\n\t{}\n\n",
                usage_info,
                self.name,
                self.description.clone().unwrap()
            )
        } else {
            format!("{}\n\n\t{}\n\n", usage_info, self.name)
        }
    }
}

/// A single argument used inside of [CLIMaker].
pub struct Argument {
    /// A short call parameter that is used with a prefix of a single hyphen (`-`).
    pub short_call: String,

    /// A long call parameter. This allows a user to enter something like
    /// `./test hello` instead of `./test --hello`.
    pub standalone_call: Option<String>,

    /// Help message (highly reccomended).
    pub help: Option<String>,

    /// Item to run when asked to execute, this should be the main usage of
    /// the argument.
    pub run: Box<dyn Fn()>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_argparse() {
        /// Inside func to hook onto inside `new_arg`
        fn example_run() {
            println!("Basic argparse working");
        }

        let new_arg = Argument {
            short_call: String::from("t"),
            standalone_call: Some(String::from("test")),
            help: None,
            run: Box::new(|| example_run()),
        };

        let cli = CLIMake {
            name: String::from("Test CLI"),
            description: None,
            args: vec![new_arg],
            none_run: None,
        };

        cli.parse_args();
    }
}
