//! climake is a minimal-dependancies library for making simple arguments. This
//! libraries aim is not features but to provide a simple way to parse arguments
//! well enough with not much more processing used than the provided [std::env]
//! from the standard library.
//! 
//! For more infomation, please see the [CLIMake] object and [Argument] to get
//! started parsing arguments using this library.

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
}

impl CLIMake {
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

            let info_help = match &arg.long_call {
                Some(long_call) => format!(
                    "  -{} / -{} ({})",
                    arg.short_call, long_call, ensured_arg_help
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
        let usage_info = "Usage: ./pleasepm [OPTIONS]";

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
    /// A short call parameter that should be 1-3 chars lowercase.
    pub short_call: String,

    /// A long call parameter. NOTE: Spaces cannot be used.
    pub long_call: Option<String>,

    /// Help message (highly reccomended).
    pub help: Option<String>,

    /// Item to run when asked to execute, this should be the main usage of
    /// the argument.
    pub run: Box<dyn Fn()>,
}
