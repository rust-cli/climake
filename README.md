# climake 

The simple, dependency-free cli library ✨

- [Crates.io](https://crates.io/crates/climake)
- [Documentation](https://docs.rs/climake)

---

This branch represents the unpublished rewrite version of climake with many advantages compared to the original version which is no longer updated!

## Example 📚

Demo of a simple package manager:

```rust
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

    for (subcommand, data) in parsed.subcommands {
        if subcommand == add {
            println!("Adding package {}..", data);
        } else if subcommand == rem {
            println!("Removing package {}..", data);
        }
    }
}
```

## Installation 🚀

Simply add the following to your `Cargo.toml` file:

```toml
[dependencies]
climake = "3.0" # note: rewrite isn't out just yet!
```

## License

Duel-licensed under both the [MIT License](https://opensource.org/licenses/MIT) ([`LICENSE-MIT`](LICENSE-MIT)) and [Apache 2.0 License](https://www.apache.org/licenses/LICENSE-2.0) ([`LICENSE-APACHE`](LICENSE-APACHE)), you may choose at your discretion.
