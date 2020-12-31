//! Contains help implementations for [CliMake]

use super::CliMake;
use crate::core::utils::writeln_term;

use std::io::Write;
use std::env;

impl<'a> CliMake<'a> {
    /// Generates header and streams to given [Write] buffer for displaying info
    /// about this cli.
    ///
    /// Please check [CliMake::help_msg] for the full help message generation used
    /// throughout automatic execution of this cli. The `usage_suffix` input used
    /// for this method is used for [Subcommand] help where the subcommand in
    /// question would like to display itself on the end of the top usage line
    /// for the header
    ///
    /// # Example
    ///
    /// What this may display:
    ///
    /// ```none
    /// Usage: ./my-app [OPTIONS]
    ///
    ///   My app v0.1.0 — A simple application
    /// ```
    pub(crate) fn header_msg(
        &self,
        usage_suffix: impl Into<Option<&'a str>>,
        buf: &mut impl Write,
    ) -> std::io::Result<()> {
        let cur_exe = env::current_exe().unwrap(); // TODO: better errors
        let cur_stem = cur_exe.file_stem().unwrap().to_str().unwrap(); // TOOD: better errors

        match usage_suffix.into() {
            Some(suffix) => {
                buf.write_fmt(format_args!("Usage: ./{} {} [OPTIONS]\n", cur_stem, suffix))?
            }
            None => buf.write_fmt(format_args!("Usage: ./{} [OPTIONS]\n", cur_stem))?,
        }

        match self.description.clone() {
            Some(d) => {
                buf.write("\n".as_bytes())?; // write formatting empty byte

                writeln_term(
                    match &self.version {
                        Some(v) => format!("{} v{} — {}", self.name, v, d),
                        None => format!("{} — {}", self.name, d),
                    },
                    buf,
                )
            }
            None => Ok(()),
        }
    }

    /// Displays help infomation for climake which is used inside the execution
    /// of the cli
    ///
    /// # Help sources
    ///
    /// This method gets sections of messaging such as the header from various
    /// *public*-available methods inside of this library:
    ///
    /// - [CliMake::header_msg]: Header generation for help message and errors
    /// - [Argument::help_name_msg]: Help generation for single [Argument]s
    ///
    /// # Example
    ///
    /// What this may look like:
    ///
    /// ```none
    /// Usage: ./my-app [OPTIONS]
    ///
    ///   My app v0.1.0 — A simple application
    ///
    /// Arguments:
    ///   (-v, --verbose) — Verbose mode
    /// ```
    pub(crate) fn help_msg(&self, buf: &mut impl Write) -> std::io::Result<()> {
        self.header_msg(None, buf)?;

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
}
