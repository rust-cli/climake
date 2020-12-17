use std::io::{prelude::*, LineWriter};
use std::{env, fmt};

/// Default help message for [Argument]s without help added
const HELP_DEFAULT: &str = "No help provided";

/// Tabs to render for CLI arguments. This will be subtracted from 80 char width
/// of terminals allowed so spaces are reccomended
const CLI_TABBING: &str = "  ";

/// A single type of call for an [Argument], can be a short call or a long call
#[derive(Debug, PartialEq)]
enum CallType {
    /// Short, single-char call, e.g. `-h`
    Short(char),

    /// Long, multi-char call, e.g. `--hello`
    Long(String),
}

impl fmt::Display for CallType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CallType::Short(c) => write!(f, "{}", c),
            CallType::Long(string) => write!(f, "--{}", string),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Argument<'a> {
    /// Many [CallType]s corrosponding to this argument
    calls: Vec<CallType>,

    /// Optional help message
    help: Option<&'a str>,
}

impl<'a> Argument<'a> {
    /// Creates a new [Argument] from given passed values
    pub fn new(
        short_calls: impl IntoIterator<Item = char>,
        long_calls: impl IntoIterator<Item = &'a str>,
        help: impl Into<Option<&'a str>>,
    ) -> Self {
        let mut calls: Vec<CallType> = short_calls
            .into_iter()
            .map(|call| CallType::Short(call))
            .collect();
        calls.append(
            &mut long_calls
                .into_iter()
                .map(|call| CallType::Long(call.to_string()))
                .collect::<Vec<CallType>>(),
        );

        Self {
            calls,
            help: help.into(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct CliMake<'a> {
    /// Internal arguments stored inside the cli once created/added to
    arguments: Vec<Argument<'a>>,

    /// Name of the program using the cli
    name: &'a str,

    /// Optional short description of the program using the cli
    description: Option<&'a str>,

    /// Optional version string of the program using the cli
    ///
    /// # Crate version
    ///
    /// If you would like this value to automatically update with your crates version,
    /// you may use a variation of the following function:
    ///
    /// ```rust
    /// pub fn crate_version() -> String {
    ///     format!(
    ///         "{}.{}.{}{}",
    ///         env!("CARGO_PKG_VERSION_MAJOR"),
    ///         env!("CARGO_PKG_VERSION_MINOR"),
    ///         env!("CARGO_PKG_VERSION_PATCH"),
    ///         option_env!("CARGO_PKG_VERSION_PRE").unwrap_or("")
    ///     )
    /// }
    /// ```
    version: Option<&'a str>,

    /// Internal/private tabbing to use, defaults to [CLI_TABBING]
    tabbing: &'static str,
}

impl<'a> CliMake<'a> {
    /// Creates a new [Argument] from given passed values
    pub fn new(
        arguments: impl Into<Vec<Argument<'a>>>,
        name: impl Into<&'a str>,
        description: impl Into<Option<&'a str>>,
        version: impl Into<Option<&'a str>>,
    ) -> Self {
        CliMake {
            arguments: arguments.into(),
            name: name.into(),
            description: description.into(),
            version: version.into(),
            tabbing: CLI_TABBING,
        }
    }

    /// Adds a single argument to this root [CliMake]
    pub fn add_arg(&mut self, argument: impl Into<Argument<'a>>) {
        self.arguments.push(argument.into())
    }

    /// Adds multiple arguments to this root [CliMake]
    pub fn add_args(&mut self, arguments: impl IntoIterator<Item = Argument<'a>>) {
        for arg in arguments.into_iter() {
            self.add_arg(arg)
        }
    }

    /// Sets tabbing distance for current [CliMake], default is `2` spaces for tabs
    pub fn tabbing(&mut self, tab_size: impl Into<&'static str>) {
        self.tabbing = tab_size.into();
    }

    /// Generates header and streams to given [Write] buffer for displaying info
    /// about this cli, see [fmt::Display] impl for full help rendering
    ///
    /// # Example
    ///
    /// ```none
    /// Usage: ./my-app [OPTIONS]
    ///
    ///   v0.1.0 — A simple application
    /// ```
    pub fn gen_header_line(&self, mut buf: impl Write) -> std::io::Result<()> {
        let cur_exe = env::current_exe();

        buf.write_fmt(format_args!(
            "Usage: ./{} [OPTIONS]\n",
            cur_exe.unwrap().file_stem().unwrap().to_str().unwrap()
        ))?;

        match self.description.clone() {
            Some(d) => {
                let newline_byte = "\n".as_bytes();

                buf.write(newline_byte)?; // write formatting empty byte

                let desc_section = match &self.version {
                    Some(v) => format!("{} v{} — {}", self.name, v, d),
                    None => format!("{} — {}", self.name, d),
                };

                let mut line_buf = LineWriter::new(buf);

                for line in desc_section.as_bytes().chunks(80 - CLI_TABBING.len()) {
                    line_buf.write(&[CLI_TABBING.as_bytes(), line, newline_byte].concat())?;
                }

                Ok(())
            }
            None => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arg_new() {
        assert_eq!(
            Argument::new(vec!['a', 'b'], vec!["hi", "there"], None),
            Argument {
                calls: vec![
                    CallType::Short('a'),
                    CallType::Short('b'),
                    CallType::Long("hi".to_string()),
                    CallType::Long("there".to_string())
                ],
                help: None
            }
        )
    }
}
