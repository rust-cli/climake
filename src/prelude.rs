//! Prelude for climake to allow easy importing of data structures
//!
//! # Contents
//!
//! This prelude is small as climake itself isn't a woefully complex library,
//! here's what this prelude includes:
//!
//! - [climake::Argument](Argument)
//! - [climake::CliMake](CliMake)
//! - [climake::Data](Data)
//! - [climake::Input](Input)
//! - [climake::Subcommand](Subcommand)
//! - [climake::parsed::ParsedArgument](ParsedArgument)
//! - [climake::parsed::ParsedCli](ParsedCli)
//! - [climake::parsed::ParsedSubcommand](ParsedSubcommand)

pub use crate::parsed::{ParsedArgument, ParsedCli, ParsedSubcommand};
pub use crate::{Argument, CliMake, Data, Input, Subcommand};
