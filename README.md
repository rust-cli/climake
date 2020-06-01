# climake

The simple, dependancy-less cli framework ✨

## Example 📚

```rust
use climake::*;

/// This will be ran when the -q (or --qwerty) argument is ran. args are the
/// arguments passed.
fn qwerty_run_me(args: Vec<String>) {
    println!(
        "The -q (or --qwerty) argument was ran! Here are the arguments passed: {:?}.",
        args
    );
}

fn other_arg_main(_args: Vec<String>) {
    println!("The normal --other or -o or -t argument.");
}

fn main() {
    let qwerty_arg = CliArgument::new(
        vec!['q'],
        vec!["qwerty"],
        Some("Some useful help info."),
        Box::new(&qwerty_run_me) // this could be any closure/func with the arg being `Vec<String>`
    );

    let other_arg = CliArgument::new(
        vec!['o', 't'],
        vec!["other"],
        None, // no help here!
        Box::new(&other_arg_main)
    );

    let cli = CliMake::new(
        vec![qwerty_arg, other_arg],
        Some("This is some help info for this example CLI.")
    ).unwrap();

    cli.parse() // runs all given parts like qwerty_run_me if called
}
```

## Installation 🚀

Simply add the following to your `Cargo.toml` file:

```toml
climake = "1.0"
```
