//! Utility items for internal crate operation

use crate::CLI_TABBING;

use std::io::{LineWriter, Write};

/// Writes a given buffer to terminal using [LineWriter] and splits every 80
/// characters, making it ideal for concise terminal displays for help messages
pub(crate) fn writeln_term(
    to_write: impl Into<String>,
    buf: &mut impl Write,
) -> std::io::Result<()> {
    let mut line_buf = LineWriter::new(buf);
    let newline_byte = "\n".as_bytes();

    for line in to_write.into().as_bytes().chunks(80 - CLI_TABBING.len()) {
        line_buf.write(&[CLI_TABBING.as_bytes(), line, newline_byte].concat())?;
    }

    Ok(())
}
