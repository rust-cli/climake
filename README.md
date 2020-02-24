# climake

## Overview

climake or CLIMake is a simple, lightweight library for native argument parsing in Rust. It is designed to run without any dependancies apart from the [standard library](). This project does not aim to include "fancy" ux-orientated features of other Rust-based argument parsers but instead just aims to ge t the job done as uniformally, lightweight and as bug-free as possible.

## Demonstration

Provided the following rust code is compiled as `./x`:

```rust
/// Inside func to hook onto inside `new_arg`
fn example_run() {
    println!("Basic argparse working");
}

let new_arg = Argument {
    short_call: String::from("t"),
    standalone_call: Some(String::from("test")),
    help: None,
    run: Box::new(|| example_run()),
};

let cli = CLIMake {
    name: String::from("Test CLI"),
    description: None,
    args: vec![new_arg],
    none_run: None,
};

cli.parse_args();
```

We can use one of the following methods to call the objective `example_run()` function:

- `./x -t`
- `./x test`

This will then output as stdout the following:

```none
Basic argparse working
```

Tada! We have got an argument parser!
