use climake::{Argument, CliMake, DataType};

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

    let cli = CliMake::new(args, Some("A showcase CLI to demonstrate climake"), None).unwrap();

    println!("Args used: {:#?}", cli.parse());
}
