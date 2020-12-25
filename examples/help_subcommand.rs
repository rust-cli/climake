//! Example showing a demonstration subcommand utilising help generation for itself
//! with semi-complex recursion/duplication of subcommands

use climake::prelude::*;
use std::io;

fn main() -> io::Result<()> {
    let subcmd_img = Subcommand::new("img", vec![], vec![], "An image");
    let subcmd_text = Subcommand::new("text", vec![], vec![], "Textual input");

    let subcmd_add = Subcommand::new(
        "add",
        vec![],
        vec![&subcmd_img, &subcmd_text],
        "Adds a new post",
    );
    let subcmd_rem = Subcommand::new(
        "rem",
        vec![],
        vec![&subcmd_img, &subcmd_text],
        "Removes an existing post",
    );

    let cli = CliMake::new(
        "subcommandsgalore",
        vec![],
        vec![&subcmd_add, &subcmd_rem],
        "A demonstration cli to show subcommand help",
        None,
    );

    println!("Help message overall:\n----");
    cli.help_msg(&mut io::stdout())?;
    println!("----\nAdd subcommand help message:\n----");
    subcmd_add.help_msg(&cli, &mut io::stdout())?;
    println!("----\nRemove subcommand help message:\n----");
    subcmd_rem.help_msg(&cli, &mut io::stdout())?;

    Ok(())
}
