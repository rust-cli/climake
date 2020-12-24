//! Example to show help generation in action. Please note that this does not
//! parse any cli infomation, but just simply prints out infomation about this cli

use climake::{Argument, CliMake, Input};
use std::io;

fn main() -> std::io::Result<()> {
    let verbose = Argument::new(
        "Toggles verbose mode",
        vec!['v'],
        vec!["verbose"],
        Input::None,
    );
    let debug = Argument::new(
        "Toggles debug infomation",
        vec!['d'],
        vec!["debug"],
        Input::None,
    );

    let cli = CliMake::new(
        "Help info",
        vec![&verbose, &debug],
        vec![],
        "A simple utility cli to print help info",
        "1.0.0",
    );

    cli.help_msg(&mut io::stdout())
}
