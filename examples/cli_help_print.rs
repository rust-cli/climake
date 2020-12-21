//! Example to show help generation in action. Please note that this does not
//! parse any cli infomation, but just simply prints out infomation about this cli

use climake::{Argument, CliMake, Input};
use std::io;

fn main() -> std::io::Result<()> {
    let arguments = vec![
        Argument::new(
            "Toggles on verbose output",
            vec!['v'],
            vec!["verbose"],
            Input::None,
        ),
        Argument::new(
            "Toggles on debug mode",
            vec!['d'],
            vec!["debug"],
            Input::None,
        ),
        Argument::new(
            "Path to load from",
            vec!['p', 'f'],
            vec!["path", "file"],
            Input::Path,
        ),
    ];

    let cli = CliMake::new(
        "Help info",
        arguments,
        vec![],
        "A simple utility cli to print help info",
        "1.0.0",
    );

    cli.help_msg(&mut io::stdout())
}
