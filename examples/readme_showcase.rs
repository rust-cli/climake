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

    println!("Args used:\n{:#?}", cli.parse());
}
