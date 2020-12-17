//! Example which simply prints header infomation. Please note that this does not
//! parse any cli infomation, just simply prints out infomation about this cli

use climake::CliMake;
use std::io;

fn main() -> io::Result<()> {
    println!("Simple:");
    CliMake::new(
        vec![],
        "Example program",
        "A simple description",
        "1.0.0"
    ).gen_header_line(io::stdout())?;

    println!("Long:");
    CliMake::new(
        vec![], 
        "Example program", 
        "A very long description, designed to potentially span multiple lines to test the prowess of formatting climake uses",
        "1.0.0"
    ).gen_header_line(io::stdout())?;

    Ok(())
}
