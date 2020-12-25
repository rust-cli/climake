//! Example which simply prints header infomation. Please note that this does not
//! parse any cli infomation, but just simply prints out infomation about this cli

use climake::prelude::*;
use std::io;

const DIVIDER: &str =
    "--------------------------------------------------------------------------------";

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    println!("Simple:\n{}", DIVIDER);
    CliMake::new(
        "Example program",
        vec![],
        vec![],
        "A simple description",
        "1.0.0",
    )
    .header_msg(None, &mut stdout)?;

    println!("{0}\n\nLong:\n{0}", DIVIDER);
    CliMake::new(
        "Example program", 
        vec![], 
        vec![], 
        "A very long description, designed to potentially span multiple lines to test the prowess of formatting climake uses",
        "1.0.0"
    ).header_msg(None, &mut stdout)?;
    println!("{}", DIVIDER);

    println!("{0}\n\nSubcommand example:\n{0}", DIVIDER);
    CliMake::new(
        "Example program", 
        vec![], 
        vec![], 
        "This program contains the subcommand aptly named w/the `my_subcommand` command added onto this",
        "1.0.0"
    ).header_msg("my_subcommand", &mut stdout)?;
    println!("{}", DIVIDER);

    Ok(())
}
