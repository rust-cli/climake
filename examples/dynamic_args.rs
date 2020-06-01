//! Simple demo of a function being dynamic in what args are allowed and printing
//! back all content given to it.

use climake::*;

/// Func to run for both args, providing the user info back on what they just
/// ran.
fn test_func(args: Vec<String>) {
    println!("It works! Found args: {:?}", args);
}

fn main() {
    let cli_args = vec![
        CliArgument::new(
            vec!['q', 'r', 's'],
            vec!["hi", "second"],
            Some("Simple help"),
            Box::new(&test_func),
        ),
        CliArgument::new(
            vec!['a', 'b', 'c'],
            vec!["other", "thing"],
            Some("Other help"),
            Box::new(&test_func),
        ),
    ]; // you can also do this with each arg being a variable on its own

    let cli = CliMake::new(cli_args, Some("A simple CLI.")).unwrap();

    cli.parse();
}
