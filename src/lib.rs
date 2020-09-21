//! The simple, dependency-less cli library ✨

#![doc(
    html_logo_url = "https://github.com/rust-cli/climake/raw/master/logo.png",
    html_favicon_url = "https://github.com/rust-cli/climake/raw/master/logo.png"
)]

use std::path::PathBuf;

// TODO: docstring
pub enum AllowedData {
    /// No allowed data
    None,
    /// Plaintext
    Plaintext(String),
    /// Single file
    File(PathBuf),
    /// Multiple files, like [AllowedData:File] but less strict
    Files(Vec<PathBuf>),
}

pub struct Argument<'arg> {
    pub short_calls: &'arg [char],
    pub long_calls: &'arg [&'arg str],
    pub help: Option<&'arg str>,
    pub allowed_data: AllowedData,
}

impl<'arg> Argument<'arg> {
    /// Shortcut to creating a new [Argument]
    pub fn new(
        short_calls: &'arg [char],
        long_calls: &'arg [&'arg str],
        help: Option<&'arg str>,
        allowed_data: AllowedData,
    ) -> Self {
        Self {
            short_calls,
            long_calls,
            help,
            allowed_data,
        }
    }

    /// Makes properly-formatted call combinations like `(-xyz, --foo, --bar)`
    fn help_combinations(&self) -> String {
        let mut fmt_long_calls = self.long_calls.join(", ");

        if self.short_calls.len() == 0 {
            format!("({})", fmt_long_calls)
        } else {
            // TODO: find a nicer way to do this
            let output_buf = vec![
                format!("-{}", self.short_calls.iter().collect::<String>()),
                fmt_long_calls,
            ];
            format!("({})", output_buf.join(", "))
        }
    }
}

// TODO: docstring
struct SubCommand<'cmd, 'arg> {
    pub name: &'cmd str,
    pub help: Option<&'cmd str>,
    pub allowed_args: Vec<Argument<'arg>>,
    pub subcommands: Vec<SubCommand<'cmd, 'arg>>,
}

// TODO: docstring
struct CliMake<'cli, 'cmd, 'arg> {
    /// Description of the program for cli help
    pub description: Option<&'cli str>,
    /// The entered [SubCommand]s
    pub sub_cmds: Vec<SubCommand<'cmd, 'arg>>,
    /// The entered [Argument]s
    pub args: Vec<Argument<'arg>>,
    /// Version of program the cli is running on. Default is the crate version but
    /// this can be changed with [CliMake::custom_version]
    version: &'cli str,
}

impl<'cli, 'cmd, 'arg> CliMake<'cli, 'cmd, 'arg> {
    /// Creates a new [CliMake] struct from arguments ([Argument]) and
    /// sub-commands ([SubCommand])
    ///
    /// If you'd like to create this struct using just args you may do so with
    /// [CliMake::from_args] or from just sub-commands with [CliMake::from_sub_cmds]
    pub fn new(
        description: Option<&'cli str>,
        args: Vec<Argument<'arg>>,
        sub_cmds: Vec<SubCommand<'cmd, 'arg>>,
    ) -> Self {
        unimplemented!();
    }

    /// Creates a [CliMake] struct from just arguments ([Argument])
    pub fn from_args(description: Option<&'cli str>, args: Vec<Argument<'arg>>) -> Self {
        unimplemented!();
    }

    /// Creates a [CliMake] struct from just sub-commands ([SubCommand])
    pub fn from_sub_cmds(sub_cmds: Vec<SubCommand<'cmd, 'arg>>) -> Self {
        unimplemented!();
    }

    /// Replaces the default crate version shown with a custom version. It is
    /// advisable to add a `v` at the start of a custom version for
    /// standardisation with other cli's
    pub fn custom_version(&mut self, version: &'cli str) {
        self.version = version
    }

    /// Parses arguments, the main duty of climake
    pub fn parse(&mut self) -> ! {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    /// Tests that [Argument::help_combinations] works correctly
    #[test]
    fn arg_help_combinations() {
        // TODO
    }
}
