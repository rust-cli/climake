//! Contains parsing implementations for [CliMake]

use super::{Argument, CliMake, Subcommand};
use crate::core::argument::CallType;
use crate::parsed::{ParsedArgument, ParsedCli, ParsedSubcommand};

use std::env;

/// Container enumeration for [crate::parsed]-related structs to be sent up the
/// chain from [match_next] recursive parsing
enum ParsedLayer<'a> {
    ParsedArgument(ParsedArgument<'a>),
    ParsedSubcommand(ParsedSubcommand<'a>),
}

/// Internal error enum representing instances of user-facing errors whilst parsing
/// (i.e. due to bad user input). These should be converted into strings and shown
/// to the user as directly as possible
enum ParseError {
    /// When a given subcommand which is being parsed in [match_next_subcommand]
    /// could not be found
    SubcommandNotFound(String),
}

/// Recurses down from an initial empty [ParsedSubcommand] to fill it in. This
/// is used as the main "entrypoint" to parsing
fn match_next_subcommand<'a>(
    inputs: &mut impl Iterator<Item = String>,
    mut parsed_subcommand: ParsedSubcommand<'a>,
) -> Result<ParsedSubcommand<'a>, ParseError> {
    loop {
        let next_input = inputs.next();

        match next_input {
            Some(input) => {
                if input.starts_with('-') {
                    // argument matched
                    // match find_argument(input, parsed_subcommand.inner.arguments) {
                    //     TODO
                    // }
                } else {
                    // subcommand matched
                    match find_subcommand(&input, &parsed_subcommand.inner.subcommands) {
                        Some(subcommand) => parsed_subcommand.subcommands.push(
                            match_next_subcommand(inputs, ParsedSubcommand::new_empty(subcommand))?,
                        ), // found subcommand, parse and add to `subcommands`
                        None => return Err(ParseError::SubcommandNotFound(input)), // subcommand was not found
                    }
                }
            }
            None => break,
        }
    }

    Ok(parsed_subcommand)
}

/// Finds `name`'d argument(s) in the passed vector of [Argument]s
fn find_argument<'a>(call: impl AsRef<str>, arguments: Vec<&'a Argument<'a>>) -> Vec<&'a Argument<'a>> {
    let mut found_arguments = vec![]; // arg output vec

    if &call.as_ref()[..2] == "--" {
        // long call matched
        let call_match = &call.as_ref()[2..];

        for argument in arguments.iter() {
            unimplemented!()
        }
    }

    found_arguments
}

/// Finds `name`'d subcommand in the passed vector of `subcommands`
fn find_subcommand<'a>(name: impl AsRef<str>, subcommands: &Vec<&'a Subcommand>) -> Option<&'a Subcommand<'a>> {
    for subcommand in subcommands.iter() {
        if name.as_ref() == subcommand.name {
            return Some(subcommand);
        }
    }

    None
}

impl<'a> CliMake<'a> {
    /// Parses all arguments from a custom iterator, see [CliMake::parse] for
    /// default parsing from [env::args]
    pub fn parse_custom(&'a self, inputs: impl IntoIterator<Item = String>) -> ParsedCli<'a> {
        unimplemented!()
    }

    /// Parses default arguments coming from [env::args]
    pub fn parse(&'a self) -> ParsedCli<'a> {
        self.parse_custom(env::args())
    }
}
