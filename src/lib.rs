use std::io::{prelude::*, LineWriter};
use std::{env, fmt};

/// Default help message for [Argument]s without help added
const HELP_DEFAULT: &str = "No help provided";

/// Tabs to render for cli arguments. This will be subtracted from 80 char width
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

/// An input type, typically given for an [Argument] to descibe what types are
/// allowed to be passwed in. This is then transferred to [Data] once the cli
/// has been executed
#[derive(Debug, PartialEq)]
pub enum Input {
    /// No input allowed, will error if any is given
    None,

    /// Text input allowed, this will return an empty string if no text is supplied
    Text,

    /// A single [PathBuf] given to the argument, these are not certain to exist
    /// and simply echo the user's input
    Path,

    /// Multiple [PathBuf]s given to the argument, these are not certain to exist
    /// and simply echo the user's input
    Paths,
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // formatting has a space on existing words on purpouse for help generation
        match self {
            Input::None => write!(f, ""),
            Input::Text => write!(f, "[text] "),
            Input::Path => write!(f, "[path] "),
            Input::Paths => write!(f, "[paths] "),
        }
    }
}

/// An argument, infomaton coming soon..
#[derive(Debug, PartialEq)]
pub struct Argument<'a> {
    /// Optional help message
    help: Option<&'a str>,

    /// Many [CallType]s corrosponding to this argument
    calls: Vec<CallType>,

    /// [Input] type allowed for this argument
    input: Input,
}

impl<'a> Argument<'a> {
    /// Creates a new [Argument] from given passed values
    pub fn new(
        help: impl Into<Option<&'a str>>,
        short_calls: impl IntoIterator<Item = char>,
        long_calls: impl IntoIterator<Item = &'a str>,
        input: impl Into<Input>,
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
            help: help.into(),
            calls,
            input: input.into(),
        }
    }

    /// Generates help message for current [Argument]
    ///
    /// This writes directly to a buffer of some kind (typically [std::io::stdout])
    /// for simplicity, perf and extendability reasons.
    ///
    /// # Example
    ///
    /// Usage:
    ///
    /// ```rust
    /// use std::io;
    /// use climake::{Argument, Input};
    ///
    /// fn main() {
    ///     let arg = Argument::new(
    ///         "Verbose mode", vec!['v'], vec!["verbose"], Input::None
    ///     );
    ///
    ///     arg.help_msg(&mut io::stdout()).unwrap();
    /// }
    /// ```
    ///
    /// What this may look like:
    ///
    /// ```none
    ///   (-v, --verbose) — Verbose mode
    /// ```
    pub fn help_msg(&self, buf: &mut impl Write) -> std::io::Result<()> {
        let mut lc_buf: Vec<String> = Vec::new();
        let mut sc_buf: Vec<char> = Vec::new();

        for call in self.calls.iter() {
            match call {
                CallType::Long(call) => lc_buf.push(format!("--{}", call)),
                CallType::Short(call) => sc_buf.push(*call),
            }
        }

        let short_calls: String = if sc_buf.len() == 0 {
            String::new()
        } else {
            format!("-{}", sc_buf.iter().collect::<String>())
        };

        let mut formatted_calls = vec![short_calls];
        formatted_calls.append(&mut lc_buf);

        let formatted_help = match self.help {
            Some(msg) => msg,
            None => HELP_DEFAULT,
        };

        writeln_term(
            if formatted_calls.len() == 1 && formatted_calls[0] != "" {
                format!("{} {}— {}", formatted_calls[0], self.input, formatted_help)
            } else {
                format!(
                    "({}) {}— {}",
                    formatted_calls.join(", "),
                    self.input,
                    formatted_help,
                )
            },
            buf,
        )
    }
}

/// Subcommand attached to a cli, allowing non-argument commands to be executed
/// with arguments attached to oneself for more complex operations
#[derive(Debug, PartialEq)]
pub struct Subcommand<'a> {
    /// Name of subcommand, used both in help and as the single calling method
    pub name: &'a str,

    /// Argument(s) attached to this [Subcommand], if any
    pub arguments: Vec<Argument<'a>>
}

impl<'a> Subcommand<'a> {
    /// Creates a new subcommand from given abstracted inputs
    pub fn new(name: impl Into<&'a str>, arguments: impl Into<Vec<Argument<'a>>>) -> Self {
        Self {
            name: name.into(),
            arguments: arguments.into()
        }
    }

    /// Displays help infomation for this subcommand specifically which is used
    /// inside the execution of the cli
    ///
    /// A referenced [CliMake] is needed for this method due to it displaying a
    /// header message using [CliMake::head_msg] with an altered usage line, as
    /// seen in the examples.
    pub fn help_msg(&self, climake: &CliMake, buf: &mut impl Write) -> std::io::Result<()> {
        climake.header_msg(self.name, buf)?;

        unimplemented!()
    }
}

/// Main cli structure, infomaton coming soon..
#[derive(Debug, PartialEq)]
pub struct CliMake<'a> {
    /// Name of the program using the cli
    name: &'a str,

    /// Internal [Argument]s stored inside the cli once created/added to
    arguments: Vec<Argument<'a>>,

    /// Internal [Subcommand]s stored inside the cli once created/added to
    subcommands: Vec<Subcommand<'a>>,

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
        name: impl Into<&'a str>,
        arguments: impl Into<Vec<Argument<'a>>>,
        subcommands: impl Into<Vec<Subcommand<'a>>>,
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
    /// about this cli.
    ///
    /// Please check [CliMake::help] for the full help message generation used
    /// throughout automatic execution of this cli. The `usage_suffix` input used
    /// for this method is used for [Subcommand] help where the subcommand in
    /// question would like to display itself on the end of the top usage line
    /// for the header
    ///
    /// # Example
    ///
    /// Usage:
    ///
    /// ```rust
    /// use std::io;
    /// use climake::CliMake;
    ///
    /// fn main() {
    ///     let cli = CliMake::new(
    ///         "My app", vec![], vec![], "A simple application", "0.1.0"
    ///     );
    ///
    ///     cli.header_msg(None, &mut io::stdout()).unwrap();
    /// }
    /// ```
    ///
    /// What this may display:
    ///
    /// ```none
    /// Usage: ./my-app [OPTIONS]
    ///
    ///   My app v0.1.0 — A simple application
    /// ```
    pub fn header_msg(&self, usage_suffix: impl Into<Option<&'a str>>, buf: &mut impl Write) -> std::io::Result<()> {
        let cur_exe = env::current_exe();

        match usage_suffix.into() { // parse suffix into usage line
            Some(suffix) => buf.write_fmt(format_args!(
                "Usage: ./{} [OPTIONS] {}\n",
                cur_exe.unwrap().file_stem().unwrap().to_str().unwrap(), suffix
            ))?,
            None => buf.write_fmt(format_args!(
                "Usage: ./{} [OPTIONS]\n",
                cur_exe.unwrap().file_stem().unwrap().to_str().unwrap()
            ))?
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
    /// - [Argument::help_msg]: Help generation for single [Argument]s
    ///
    /// # Example
    ///
    /// Usage:
    ///
    /// ```rust
    /// use std::io;
    /// use climake::{CliMake, Argument, Input};
    ///
    /// fn main() {
    ///     let args = vec![
    ///         Argument::new("Verbose mode", vec!['v'], vec!["verbose"], Input::None)
    ///     ];
    ///
    ///     let cli = CliMake::new(
    ///         "My app", args, vec![], "A simple application", "0.1.0"
    ///     );
    ///
    ///     cli.help_msg(&mut io::stdout()).unwrap();
    /// }
    /// ```
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
    pub fn help_msg(&self, buf: &mut impl Write) -> std::io::Result<()> {
        self.header_msg(None, buf)?;

        buf.write("\nArguments:\n".as_bytes())?;

        if self.arguments.len() > 0 {
            for argument in self.arguments.iter() {
                argument.help_msg(buf)?;
            }
        } else {
            buf.write("  No arguments found\n".as_bytes())?;
        }

        buf.write("\nSubcommands:\n".as_bytes())?;

        if self.subcommands.len() > 0 {
            for subcommand in self.subcommands.iter() {
                subcommand.help_msg(&self, buf)?;
            }
        } else {
            buf.write("  No subcommands found\n".as_bytes())?;
        }

        Ok(())
    }
}

/// Writes a given buffer to terminal using [LineWriter] and splits every 80
//// characters, making it ideal for concise terminal displays for help messages
fn writeln_term(to_write: impl Into<String>, buf: impl Write) -> std::io::Result<()> {
    let mut line_buf = LineWriter::new(buf);
    let newline_byte = "\n".as_bytes();

    for line in to_write.into().as_bytes().chunks(80 - CLI_TABBING.len()) {
        line_buf.write(&[CLI_TABBING.as_bytes(), line, newline_byte].concat())?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arg_new() {
        assert_eq!(
            Argument::new(None, vec!['a', 'b'], vec!["hi", "there"], Input::Text),
            Argument {
                calls: vec![
                    CallType::Short('a'),
                    CallType::Short('b'),
                    CallType::Long("hi".to_string()),
                    CallType::Long("there".to_string())
                ],
                help: None,
                input: Input::Text
            }
        )
    }

    #[test]
    fn arg_full_help() -> std::io::Result<()> {
        let mut chk_vec: Vec<u8> = vec![];

        Argument::new(None, vec![], vec![], Input::None).help_msg(&mut chk_vec)?;
        assert_eq!(
            std::str::from_utf8(chk_vec.as_slice()).unwrap(),
            "  () — No help provided\n"
        );
        chk_vec = vec![];

        Argument::new("Some simple help", vec!['a'], vec!["long"], Input::Text)
            .help_msg(&mut chk_vec)?;
        assert_eq!(
            std::str::from_utf8(chk_vec.as_slice()).unwrap(),
            "  (-a, --long) [text] — Some simple help\n"
        );
        chk_vec = vec![];

        Argument::new(None, vec!['a'], vec![], Input::Text).help_msg(&mut chk_vec)?;
        assert_eq!(
            std::str::from_utf8(chk_vec.as_slice()).unwrap(),
            "  -a [text] — No help provided\n"
        );

        Ok(())
    }
}
