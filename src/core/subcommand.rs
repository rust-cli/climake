//! Contains [Subcommand]-related items, see specific documentation for more
//! information

use super::utils::writeln_term;
use super::{Argument, CliMake};
use crate::HELP_DEFAULT;

use std::io::Write;

/// A subcommand attached to the cli, allowing commands and sections of the cli
/// to form
#[derive(Debug, PartialEq, Clone)]
pub struct Subcommand<'a> {
    /// Name of subcommand, used both in help and as the single calling method
    pub name: &'a str,

    /// Argument(s) attached to this [Subcommand], if any
    pub arguments: Vec<&'a Argument<'a>>,

    /// Recursive subcommands attached to this [Subcommand], if any
    pub subcommands: Vec<&'a Subcommand<'a>>,

    /// Optional short description of this subcommand
    pub help: Option<&'a str>,
}

impl<'a> Subcommand<'a> {
    /// Creates a new subcommand from given abstracted inputs
    pub fn new(
        name: impl Into<&'a str>,
        arguments: impl Into<Vec<&'a Argument<'a>>>,
        subcommands: impl Into<Vec<&'a Subcommand<'a>>>,
        help: impl Into<Option<&'a str>>,
    ) -> Self {
        Self {
            name: name.into(),
            arguments: arguments.into(),
            subcommands: subcommands.into(),
            help: help.into(),
        }
    }

    /// Displays help infomation for this subcommand specifically which is used
    /// inside the execution of the cli
    ///
    /// A referenced [CliMake] is needed for this method due to it displaying a
    /// header message using [CliMake::header_msg] with an altered usage line, as
    /// seen in the examples.
    pub(crate) fn help_msg(&self, climake: &CliMake, buf: &mut impl Write) -> std::io::Result<()> {
        climake.header_msg(self.name, buf)?;

        match self.help {
            Some(help) => {
                buf.write("\nAbout:\n".as_bytes())?;
                writeln_term(help, buf)?;
            }
            None => (),
        };

        // TODO: merge this into a utility func shared with CliMake::help_msg
        buf.write("\nArguments:\n".as_bytes())?;

        if self.arguments.len() > 0 {
            for argument in self.arguments.iter() {
                argument.help_name_msg(buf)?;
            }
        } else {
            buf.write("  No arguments found\n".as_bytes())?;
        }

        buf.write("\nSubcommands:\n".as_bytes())?;

        if self.subcommands.len() > 0 {
            for subcommand in self.subcommands.iter() {
                subcommand.help_name_msg(buf)?;
            }
        } else {
            buf.write("  No subcommands found\n".as_bytes())?;
        }

        Ok(())
    }

    /// Generates compact help message for current [Subcommand]
    ///
    /// This writes directly to a buffer of some kind (typically [std::io::stdout])
    /// for simplicity, perf and extendability reasons.
    ///
    /// # Example
    ///
    /// What this may look like:
    ///
    /// ```none
    ///   example — A simple example subcommand
    /// ```
    pub(crate) fn help_name_msg(&self, buf: &mut impl Write) -> std::io::Result<()> {
        let formatted_help = match self.help {
            Some(msg) => msg,
            None => HELP_DEFAULT,
        };

        writeln_term(format!("{} — {}", self.name, formatted_help), buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that the [Subcommand::help_name_msg] method works correctly
    #[test]
    fn name_help() -> std::io::Result<()> {
        let mut chk_vec: Vec<u8> = vec![];

        Subcommand::new("command", vec![], vec![], "A simple command")
            .help_name_msg(&mut chk_vec)?;
        assert_eq!(
            std::str::from_utf8(chk_vec.as_slice()).unwrap(),
            "  command — A simple command\n"
        );

        Ok(())
    }
}
