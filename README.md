# climake

The simple, dependency-less cli library ✨

- [Crates.io](https://crates.io/crates/climake)
- [Documentation](https://docs.rs/climake)

## Example 📚

```rust
use climake::{Argument, CLIMake, DataType};

fn main() {
    let args = &[
        Argument::new(
            &['o'],
            &["output", "out"],
            Some("Example output arg"),
            DataType::Files,
        ).unwrap(),
        Argument::new(
            &['a', 'b', 'c'],
            &[],
            Some("Alphabet!"),
            DataType::None,
        ).unwrap(),
    ];

    let cli = CLIMake::new(args, Some("A showcase CLI to demonstrate climake"), None).unwrap();

    println!("Args used: {:#?}", cli.parse());
}
```

## Installation 🚀

Simply add the following to your `Cargo.toml` file:

```toml
[dependencies]
climake = "2.1"
```

## License

Duel-licensed under both the [MIT License](https://opensource.org/licenses/MIT) ([`LICENSE-MIT`](LICENSE-MIT)) and [Apache 2.0 License](https://www.apache.org/licenses/LICENSE-2.0) ([`LICENSE-APACHE`](LICENSE-APACHE)), you may choose at your discretion.
