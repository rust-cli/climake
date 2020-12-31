//! Contains [Argument]-related items, see specific documentation for more information

use super::utils::writeln_term;
use crate::io::Input;
use crate::HELP_DEFAULT;

use std::fmt;
use std::io::Write;

/// An argument attached to the cli, allowing passing of user data to the top-level
/// cli or subcommands
#[derive(Debug, PartialEq, Clone)]
pub struct Argument<'a> {
    /// Optional help message
    help: Option<&'a str>,

    /// Many [CallType]s corrosponding to this argument
    calls: Vec<CallType>,

    /// [Input] type allowed for this argument
    input: Input,

    /// Required argument for given root cli or [Subcommand](crate::Subcommand).
    /// If this argument is not present whilst the cli parses, it will provide an
    /// apt error
    ///
    /// To change the default behaviour of `false` (not required), simply modify
    /// this value before it's time to parse.
    required: bool,
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
            required: false,
        }
    }

    /// Adds a single short call, chainable
    pub fn add_scall(&mut self, short_call: impl Into<char>) -> &mut Self {
        self.calls.push(short_call.into().into());
        self
    }

    /// Adds multiple short calls, chainable
    pub fn add_scalls(&mut self, short_calls: impl IntoIterator<Item = char>) -> &mut Self {
        for c in short_calls.into_iter() {
            self.add_scall(c);
        }
        self
    }

    /// Adds a single long call, chainable
    pub fn add_lcall(&mut self, long_call: impl Into<String>) -> &mut Self {
        self.calls.push(long_call.into().into());
        self
    }

    /// Adds multiple long calls, chainable
    pub fn add_lcalls(&mut self, long_calls: impl IntoIterator<Item = String>) -> &mut Self {
        for c in long_calls.into_iter() {
            self.add_lcall(c);
        }
        self
    }

    /// Generates compact help message for current [Argument]
    ///
    /// This writes directly to a buffer of some kind (typically [std::io::stdout])
    /// for simplicity, perf and extendability reasons.
    ///
    /// # Example
    ///
    /// What this may look like:
    ///
    /// ```none
    ///   (-v, --verbose) — Verbose mode
    /// ```
    pub(crate) fn help_name_msg(&self, buf: &mut impl Write) -> std::io::Result<()> {
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
        let required_msg = if self.required { "[REQUIRED] " } else { "" };

        writeln_term(
            if formatted_calls.len() == 1 && formatted_calls[0] != "" {
                format!(
                    "{} {}{}— {}",
                    formatted_calls[0], self.input, required_msg, formatted_help
                )
            } else {
                format!(
                    "({}) {}{}— {}",
                    formatted_calls.join(", "),
                    self.input,
                    required_msg,
                    formatted_help,
                )
            },
            buf,
        )
    }
}

/// A single type of call for an [Argument], can be a short call or a long call
#[derive(Debug, PartialEq, Clone)]
pub(crate) enum CallType {
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

impl From<CallType> for String {
    fn from(calltype: CallType) -> Self {
        match calltype {
            CallType::Short(c) => String::from(c),
            CallType::Long(string) => string,
        }
    }
}

impl From<char> for CallType {
    fn from(c: char) -> Self {
        CallType::Short(c)
    }
}

impl From<String> for CallType {
    fn from(string: String) -> Self {
        CallType::Long(string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that the [Argument::new] method (creation of arguments) works correctly
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
                input: Input::Text,
                required: false,
            }
        )
    }

    /// Checks that the [Argument::help_name_msg] method works correctly
    #[test]
    fn name_help() -> std::io::Result<()> {
        let mut chk_vec: Vec<u8> = vec![];

        Argument::new(None, vec![], vec![], Input::None).help_name_msg(&mut chk_vec)?;
        assert_eq!(
            std::str::from_utf8(chk_vec.as_slice()).unwrap(),
            "  () — No help provided\n"
        );
        chk_vec = vec![];

        Argument::new("Some simple help", vec!['a'], vec!["long"], Input::Text)
            .help_name_msg(&mut chk_vec)?;
        assert_eq!(
            std::str::from_utf8(chk_vec.as_slice()).unwrap(),
            "  (-a, --long) [text] — Some simple help\n"
        );
        chk_vec = vec![];

        Argument::new(None, vec!['a'], vec![], Input::Text).help_name_msg(&mut chk_vec)?;
        assert_eq!(
            std::str::from_utf8(chk_vec.as_slice()).unwrap(),
            "  -a [text] — No help provided\n"
        );

        Ok(())
    }

    /// Checks that the [Argument::help_name_msg] method works correctly with [Argument::required]
    /// set to `true`
    #[test]
    fn name_help_required() -> std::io::Result<()> {
        let mut chk_vec: Vec<u8> = vec![];

        let mut arg = Argument::new("Some argument", vec!['s'], vec![], Input::None);
        arg.required = true;
        arg.help_name_msg(&mut chk_vec)?;
        assert_eq!(
            std::str::from_utf8(chk_vec.as_slice()).unwrap(),
            "  -s [REQUIRED] — Some argument\n"
        );

        Ok(())
    }
    /// Checks that the [Argument::add_scall] method works correctly
    #[test]
    fn add_scall() {
        let mut arg = Argument::new("example", vec![], vec![], Input::None);

        arg.add_scall('a').add_scall('b').add_scall('c');

        assert_eq!(
            arg,
            Argument::new("example", vec!['a', 'b', 'c'], vec![], Input::None)
        )
    }

    /// Checks that the [Argument::add_scalls] method works correctly
    #[test]
    fn add_scalls() {
        let mut arg = Argument::new("example", vec![], vec![], Input::None);

        arg.add_scalls(vec!['a', 'b']).add_scalls(vec!['c']);

        assert_eq!(
            arg,
            Argument::new("example", vec!['a', 'b', 'c'], vec![], Input::None)
        )
    }

    /// Checks that the [Argument::add_lcall] method works correctly
    #[test]
    fn add_lcall() {
        let mut arg = Argument::new("example", vec![], vec![], Input::None);

        arg.add_lcall("a").add_lcall("b").add_lcall("c");

        assert_eq!(
            arg,
            Argument::new("example", vec![], vec!["a", "b", "c"], Input::None)
        )
    }

    /// Checks that the [Argument::add_lcalls] method works correctly
    #[test]
    fn add_lcalls() {
        let mut arg = Argument::new("example", vec![], vec![], Input::None);

        arg.add_lcalls(vec!["a".to_string(), "b".to_string()])
            .add_lcalls(vec!["c".to_string()]);

        assert_eq!(
            arg,
            Argument::new("example", vec![], vec!["a", "b", "c"], Input::None)
        )
    }

    /// Checks that the [From]<[CallType]> implementation for [String] works correctly
    #[test]
    fn string_from_calltype() {
        assert_eq!(String::from(CallType::Short('h')), "h".to_string());
        assert_eq!(
            String::from(CallType::Long("testing".to_string())),
            "testing".to_string()
        );
    }
}
