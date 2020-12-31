//! Contains parsing implementations for [CliMake]

use super::CliMake;
use crate::parsed::ParsedCli;

use std::env;

impl<'a> CliMake<'a> {
    /// Parses all arguments from a custom iterator, see [CliMake::parse] for
    /// default parsing from [env::args]
    pub fn parse_custom(&'a self, arguments: impl IntoIterator<Item = String>) -> ParsedCli<'a> {
        // for argument in arguments.into_iter() {}
        unimplemented!()
    }

    /// Parses default arguments coming from [env::args]
    pub fn parse(&'a self) -> ParsedCli<'a> {
        self.parse_custom(env::args())
    }
}
