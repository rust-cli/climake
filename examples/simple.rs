//! Demo of a simple package manager

use climake::prelude::*;

fn main() {
    let package = Argument::new(
        "The package name",
        vec!['p', 'i'],
        vec!["pkg, package"],
        Input::Text,
    );

    let add = Subcommand::new("add", vec![&package], vec![], "Adds a package");
    let rem = Subcommand::new("rem", vec![&package], vec![], "Removes a package");

    let cli = CliMake::new(
        "MyPkg",
        vec![],
        vec![&add, &rem],
        "A simple package manager demo",
        "1.0.0",
    );

    let parsed = cli.parse();

    for subcommand in parsed.subcommands {
        if subcommand.inner == &add {
            println!("Adding package {:?}..", subcommand.arguments[0]);
        } else if subcommand.inner == &rem {
            println!("Removing package {:?}..", subcommand.arguments[0]);
        }
    }
}
