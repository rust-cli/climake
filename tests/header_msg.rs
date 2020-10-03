//! Tests cli header message

use climake::CliMake;

/// Ensures header message displays without errors.
///
/// *This is not checked with any [assert_eq] as header messages change with
/// binary name*
#[test]
fn check_header() {
    let cli = CliMake::new(&[], Some("A simple CLI."), None).unwrap();

    cli.header_msg();
}
