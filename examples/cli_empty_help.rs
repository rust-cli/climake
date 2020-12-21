//! Example of the most empty possible CLI you can make, to showcase edge case
//! handling in regards to help generation

use climake::CliMake;
use std::io;

fn main() -> io::Result<()> {
    let cli = CliMake::new("my-cli", vec![], vec![], None, None);

    cli.help_msg(&mut io::stdout())
}
