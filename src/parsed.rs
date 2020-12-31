//! Structures that allow the containing of completed parsing from the cli
//!
//! # Importing
//!
//! This module is included in [crate::prelude] by default so no extra importing
//! steps are required (unless you are importing explicit items).

use crate::io::Data;
use crate::{Argument, Subcommand};

/// Used argument stemming from [CliMake::parse]-related parsing
///
/// This structure contains a reference to the underlying argument and data passed
/// by user (if any).
///
/// # Implementations
///
/// This structure may be converted into a raw [Argument] with the use of the
/// [From]<[ParsedArgument]> implementation or similarly to the [Data] used for
/// this argument.
#[derive(Debug, PartialEq, Clone)]
pub struct ParsedArgument<'a> {
    /// Reference to the argument used
    pub inner: &'a Argument<'a>,

    /// Passed data for this argument
    pub data: Data,
}

impl<'a> From<ParsedArgument<'a>> for &'a Argument<'a> {
    fn from(parsed_argument: ParsedArgument<'a>) -> Self {
        parsed_argument.inner
    }
}

impl<'a> From<ParsedArgument<'a>> for Data {
    fn from(parsed_argument: ParsedArgument<'a>) -> Self {
        parsed_argument.data
    }
}

/// Used subcommand stemming from [CliMake::parse]-related parsing
///
/// This strcuture contains a reference to the underlying subcommand and all other
/// subcommands/arguments below that in a similar [ParsedSubcommand]/[ParsedArgument]
/// recursion.
///
/// # Implementations
///
/// This structure may be converted into a raw [Subcommand] with the use of the
/// [From]<[ParsedSubcommand]> implementation or similarly the [ParsedSubcommand::subcommands]
/// and [ParsedSubcommand::arguments] vectors.
#[derive(Debug, PartialEq, Clone)]
pub struct ParsedSubcommand<'a> {
    /// Reference to the subcommand used
    pub inner: &'a Subcommand<'a>,

    /// Used subcommands contained inside of this subcommand (if any)
    pub subcommands: Vec<ParsedSubcommand<'a>>,

    /// Used arguments contained inside of this subcommand (if any)
    pub arguments: Vec<ParsedArgument<'a>>,
}

impl<'a> From<ParsedSubcommand<'a>> for &'a Subcommand<'a> {
    fn from(parsed_subcommand: ParsedSubcommand<'a>) -> Self {
        parsed_subcommand.inner
    }
}

impl<'a> From<ParsedSubcommand<'a>> for Vec<ParsedSubcommand<'a>> {
    fn from(parsed_subcommand: ParsedSubcommand<'a>) -> Self {
        parsed_subcommand.subcommands
    }
}

impl<'a> From<ParsedSubcommand<'a>> for Vec<ParsedArgument<'a>> {
    fn from(parsed_subcommand: ParsedSubcommand<'a>) -> Self {
        parsed_subcommand.arguments
    }
}

/// Similar to [ParsedSubcommand], contains the top-level parsed arguments from
/// [CliMake::parse]-related parsing
///
/// # Implementations
///
/// This structure may be converted into a vector of subcommands from [ParsedCli::subcommands]
/// or arguments from [ParsedCli::arguments].
#[derive(Debug, PartialEq, Clone)]
pub struct ParsedCli<'a> {
    /// Used subcommands contained inside of top-level parsed
    pub subcommands: Vec<ParsedSubcommand<'a>>,

    /// Used arguments contained inside of top-level parsed
    pub arguments: Vec<ParsedArgument<'a>>,
}

impl<'a> From<ParsedCli<'a>> for Vec<ParsedSubcommand<'a>> {
    fn from(used_cli: ParsedCli<'a>) -> Self {
        used_cli.subcommands
    }
}

impl<'a> From<ParsedCli<'a>> for Vec<ParsedArgument<'a>> {
    fn from(used_cli: ParsedCli<'a>) -> Self {
        used_cli.arguments
    }
}
