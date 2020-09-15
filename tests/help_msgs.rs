//! Tests various types of help messages for correct output

use climake::{Argument, CLIMake, DataType};

/// Internal helper for tests that removes first `lines` lines from given
/// [String] `input`
fn remove_lines(input: String, lines: usize) -> String {
    input.split("\n").collect::<Vec<&str>>()[lines..].join("\n")
}

/// Tests individual arg's `pretty_help` message
#[test]
fn check_arg_help() {
    let arg_1 = Argument::new(
        vec!['q', 'r', 's'],
        vec![String::from("hi"), String::from("second")],
        Some("Simple help"),
        DataType::None,
    )
    .unwrap();
    let arg_2 = Argument::new(
        vec!['a', 'b', 'c'],
        vec![String::from("other"), String::from("thing")],
        Some("Other help"),
        DataType::None,
    )
    .unwrap();
    let arg_3 = Argument::new(vec!['o'], vec![], None, DataType::None).unwrap();

    assert_eq!(
        arg_1.pretty_help(),
        "\n  (-q, -r, -s, --hi, --second): Simple help"
    );
    assert_eq!(
        arg_2.pretty_help(),
        "\n  (-a, -b, -c, --other, --thing): Other help"
    );
    assert_eq!(arg_3.pretty_help(), "\n  (-o): No help message provided");
}

/// Checks that the cli can parse a full help message compared to a correct
/// help message
#[test]
fn cli_full_help() {
    const TRUE_HELP: &str = "  A simple debug cli\n\nOptions:\n  (-q, -r, --hi): Simple help\n  (-o, --2nd, --arg): A simple second arg";

    let cli_args = vec![
        Argument::new(
            vec!['q', 'r'],
            vec![String::from("hi")],
            Some("Simple help"),
            DataType::None,
        )
        .unwrap(),
        Argument::new(
            vec!['o'],
            vec![String::from("2nd"), String::from("arg")],
            Some("A simple second arg"),
            DataType::None,
        )
        .unwrap(),
    ];
    let cli = CLIMake::new(cli_args, Some("A simple debug cli"), None).unwrap();

    assert_eq!(remove_lines(cli.help_msg(), 2), TRUE_HELP);
}

/// Checks that args return proper specific help messages
#[test]
fn specific_arg_help() {
    const TRUE_HELP: &str = "Arg help:\n  (-t): Specific help";

    let arg = Argument::new(vec!['t'], vec![], Some("Specific help"), DataType::None).unwrap();
    let cli = CLIMake::new(vec![arg.clone()], None, None).unwrap();

    assert_eq!(remove_lines(cli.specific_help(&arg), 4), TRUE_HELP);
}
