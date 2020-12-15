use std::{env, fmt};

/// Default help message for [Argument]s without help added
const HELP_DEFAULT: &str = "No help provided";

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
pub struct Argument {
    /// Many [CallType]s corrosponding to this argument
    calls: Vec<CallType>,

    /// Optional help message
    help: Option<String>,
}

impl Argument {
    /// Creates a new [Argument] from given passed values
    pub fn new(
        short_calls: impl IntoIterator<Item = impl Into<char>>,
        long_calls: impl IntoIterator<Item = impl AsRef<str>>,
        help: impl Into<Option<String>>,
    ) -> Self {
        let mut calls: Vec<CallType> = short_calls
            .into_iter()
            .map(|call| CallType::Short(call.into()))
            .collect();
        calls.append(
            &mut long_calls
                .into_iter()
                .map(|call| CallType::Long(call.as_ref().to_string()))
                .collect::<Vec<CallType>>(),
        );

        Self {
            calls,
            help: help.into(),
        }
    }

    /// Renders help string (i.e. passed `help` message), see [fmt::Display] impl
    /// for full help rendering
    pub fn help_sting(&self) -> &str {
        match &self.help {
            Some(help) => help,
            None => HELP_DEFAULT,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct CliMake {
    arguments: Vec<Argument>,
    description: Option<String>,
    version: Option<String>,
}

impl CliMake {
    /// Creates a new [Argument] from given passed values
    pub fn new(
        arguments: impl Into<Vec<Argument>>,
        description: impl Into<Option<String>>,
        version: impl Into<Option<String>>,
    ) -> Self {
        CliMake {
            arguments: arguments.into(),
            description: description.into(),
            version: version.into(),
        }
    }

    /// Adds a single argument to this root [CliMake]
    pub fn add_arg(&mut self, argument: impl Into<Argument>) {
        self.arguments.push(argument.into())
    }

    /// Adds multiple arguments to this root [CliMake]
    pub fn add_args(&mut self, arguments: impl IntoIterator<Item = Argument>) {
        for arg in arguments.into_iter() {
            self.add_arg(arg)
        }
    }

    /// Generates header for displaying infomation about this cli
    ///
    /// # Example
    ///
    /// ```none
    /// Usage: ./my-app [OPTIONS]
    ///
    ///   v0.1.0 — A simple application
    /// ```
    pub fn gen_header(&self) -> String {
        let cur_exe = env::current_exe();
        let top_line = format!(
            "Usage: ./{} [OPTIONS]",
            cur_exe.unwrap().file_stem().unwrap().to_str().unwrap()
        );

        match self.description.clone() {
            Some(d) => {
                let desc_line = match &self.version {
                    Some(v) => format!("v{} — {}", v, d),
                    None => d,
                };

                format!("{}\n\n  {}", top_line, desc_line)
            }
            None => top_line,
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
