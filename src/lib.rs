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

    /// This occurs when user tries to overwrite the non-movable help command
    /// that is automatically generated. This is currently not passable if using
    /// `-h` or `--help` or `help`.
    HelpOverwrite,
}

/// Main structure for climake, the [CLIMake] object. Further arguments
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
    pub fn parse_args(&mut self) {
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
        } else if check_args(String::from("-h"))
            || check_args(String::from("--help"))
            || check_args(String::from("help"))
        {
            println!("{}", self.help_msg());
            std::process::exit(0);
        }

        // below block only changed on valid arg
        let mut valid_count = false; // checks valid args
        let mut valid_ind = 0; // similar to an enumerate

        for arg in self.args.iter() {
            let short_call_pass = match &arg.short_call {
                Some(x) => check_args(format!("-{}", x)),
                None => false,
            };

            let standalone_call_pass = match &arg.standalone_call {
                Some(x) => check_args(x.clone()),
                None => false,
            };

            if short_call_pass || standalone_call_pass {
                valid_count = true;
                valid_ind += 1;

                let run_args = match arg.got_param {
                    true => {
                        if passed_args.len() < valid_ind + 1 {
                            println!("{}No body given for argument!", self.header_text());
                            std::process::exit(1); // TODO: Fix
                        }

                        Some(passed_args[valid_ind + 1].clone()) // TODO: Fix
                    }
                    false => None,
                };

                (arg.run)(run_args);
            }
        }

        if !valid_count {
            println!("{}Argument(s) passed are invalid!", self.header_text());
            std::process::exit(1);
        }

        std::process::exit(0); // exited successfully
    }

    /// Adds a new argument to parser.
    pub fn add_existing_arg(&mut self, new_arg: Argument) -> Result<(), CLIMakeError> {
        let short_override_check = match &new_arg.short_call {
            Some(x) => x == &'h',
            None => false,
        };

        let standalone_override_check = match &new_arg.standalone_call {
            Some(x) => x == "help",
            None => false,
        };

        if short_override_check || standalone_override_check {
            return Err(CLIMakeError::HelpOverwrite);
        }

        self.args.push(new_arg);

        Ok(())
    }

    /// Displays help message in `stdout` using added arguments.
    pub fn help_msg(&self) -> String {
        let header_text = self.header_text();
        let mut generated_help = format!(
            "{}Options:\n  -h  help  --help\t | Shows this message\n",
            header_text
        );

        if self.args.len() == 0 {
            return format!("{}No arguments made!", header_text);
        }

        for arg in self.args.iter() {
            let mut arg_help = match arg.short_call {
                Some(call) => format!("  -{}", call),
                None => String::new(),
            };

            if arg.standalone_call.is_some() {
                arg_help.push_str(&format!("  {}", arg.standalone_call.clone().unwrap()));
            }

            if arg.got_param {
                arg_help.push_str(" [PARAM]");
            } else {
                arg_help.push_str("\t")
            }

            let ensured_arg_help = match &arg.help {
                Some(help) => String::clone(help),
                None => String::from("[Help not provided]"),
            };

            arg_help.push_str(&format!("\t | {}\n", ensured_arg_help));

            generated_help.push_str(&arg_help);
        }

        generated_help
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
    pub short_call: Option<char>,

    /// A long call parameter. This allows a user to enter something like
    /// `./test hello` instead of `./test --hello`.
    pub standalone_call: Option<String>,

    /// Allows it to capture next element inside of arguments. This is
    /// experimental and can be buggy if you do something like `hello hello`
    /// that will go on forever.
    pub got_param: bool,

    /// Help message (highly reccomended).
    pub help: Option<String>,

    /// Item to run when asked to execute, this should be the main usage of
    /// the argument. The [Option]<[String]> is linked to [Argument::got_param].
    /// If got_param is `true`, there will always be [String] present, even if
    /// inside of an [Option].
    pub run: Box<dyn Fn(Option<String>)>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_argparse() {
        /// Inside func to hook onto inside `new_arg`
        fn example_run(_arg: Option<String>) {
            println!("Basic argparse working");
        }

        let new_arg = Argument {
            short_call: Some('t'),
            got_param: false,
            standalone_call: Some(String::from("test")),
            help: None,
            run: Box::new(example_run),
        };

        let mut cli = CLIMake {
            name: String::from("Test CLI"),
            description: None,
            args: vec![new_arg],
            none_run: None,
        };

        cli.parse_args();
    }
}
