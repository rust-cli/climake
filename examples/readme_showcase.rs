use climake::{Argument, CLIMake, DataType};

fn main() {
    let args = vec![
        Argument::new(
            vec!['o'],
            vec![String::from("output"), String::from("out")],
            Some("Example output arg"),
            DataType::File,
        ).unwrap(),
        Argument::new(
            vec!['a', 'b', 'c'],
            vec![],
            Some("Alphabet!"),
            DataType::None,
        ).unwrap(),
    ];

    let cli = CLIMake::new(args, Some("A showcase CLI to demonstrate climake"), None).unwrap();

    println!("Args used:\n{:#?}", cli.parse());
}
