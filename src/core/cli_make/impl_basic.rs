//! Contains basic implementations for [CliMake]

use super::CliMake;
use crate::{Argument, Subcommand, CLI_TABBING};

impl<'a> CliMake<'a> {
    /// Creates a new [Argument] from given passed values
    pub fn new(
        name: impl Into<&'a str>,
        arguments: impl Into<Vec<&'a Argument<'a>>>,
        subcommands: impl Into<Vec<&'a Subcommand<'a>>>,
        description: impl Into<Option<&'a str>>,
        version: impl Into<Option<&'a str>>,
    ) -> Self {
        CliMake {
            name: name.into(),
            arguments: arguments.into(),
            subcommands: subcommands.into(),
            description: description.into(),
            version: version.into(),
            tabbing: CLI_TABBING,
        }
    }

    /// Adds a single argument to this root [CliMake], chainable
    pub fn add_arg(&mut self, argument: impl Into<&'a Argument<'a>>) -> &mut Self {
        self.arguments.push(argument.into());
        self
    }

    /// Adds multiple arguments to this root [CliMake], chainable
    pub fn add_args(&mut self, arguments: impl IntoIterator<Item = &'a Argument<'a>>) -> &mut Self {
        for arg in arguments.into_iter() {
            self.add_arg(arg);
        }
        self
    }

    /// Adds a single subcommand to this root [CliMake], chainable
    pub fn add_subcmd(&mut self, subcommand: impl Into<&'a Subcommand<'a>>) -> &mut Self {
        self.subcommands.push(subcommand.into());
        self
    }

    /// Adds multiple subcommands to this root [CliMake], chainable
    pub fn add_subcmds(
        &mut self,
        subcommands: impl IntoIterator<Item = &'a Subcommand<'a>>,
    ) -> &mut Self {
        for subcommand in subcommands.into_iter() {
            self.add_subcmd(subcommand);
        }
        self
    }

    /// Sets the tabbing characters for cli help, the default for this is 2 spaces,
    /// i.e. `  `.
    pub fn tabbing(&mut self, tab_chars: &'static str) -> &mut Self {
        self.tabbing = tab_chars;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::Input;

    /// Checks that the [CliMake::add_arg] method works correctly
    #[test]
    fn cli_add_arg() {
        let mut cli = CliMake::new("example", vec![], vec![], "Add arg check", None);
        let arg = Argument::new("arg help", vec![], vec![], Input::None);

        cli.add_arg(&arg).add_arg(&arg);

        assert_eq!(cli.arguments, vec![&arg, &arg])
    }

    /// Checks that the [CliMake::add_args] method works correctly
    #[test]
    fn cli_add_args() {
        let mut cli = CliMake::new("example", vec![], vec![], "Add arg check", None);
        let arg = Argument::new("arg help", vec![], vec![], Input::None);

        cli.add_args(vec![&arg, &arg]).add_args(vec![&arg, &arg]);

        assert_eq!(cli.arguments, vec![&arg, &arg, &arg, &arg])
    }

    /// Checks that the [CliMake::add_subcmds] method works correctly
    #[test]
    fn cli_add_subcmds() {
        let mut cli = CliMake::new("example", vec![], vec![], "Add arg check", None);
        let subcmd = Subcommand::new("example", vec![], vec![], None);

        cli.add_subcmds(vec![&subcmd, &subcmd])
            .add_subcmds(vec![&subcmd, &subcmd]);

        assert_eq!(cli.subcommands, vec![&subcmd, &subcmd, &subcmd, &subcmd])
    }

    /// Checks that the [CliMake::add_subcmd] method works correctly
    #[test]
    fn cli_add_subcmd() {
        let mut cli = CliMake::new("example", vec![], vec![], "Add arg check", None);
        let subcmd = Subcommand::new("example", vec![], vec![], None);

        cli.add_subcmd(&subcmd).add_subcmd(&subcmd);

        assert_eq!(cli.subcommands, vec![&subcmd, &subcmd])
    }
}
