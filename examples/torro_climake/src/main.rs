use climake::{Argument, CLIError, CLIMake, DataType, PassedData};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use torro::bencode;
use torro::torrent::Torrent;

/// Completes activities once torrent file is correctly passed after a `-t` arg
fn do_torrent(torrent_path: PathBuf) {
    println!(
        "Torrent creation result:\n{:#?}",
        Torrent::from_file(torrent_path)
    )
}

/// Completes activities once bencoded torrent file is correctly passed after a
/// `-b` arg
fn do_bencode(bencode_path: PathBuf) {
    let file_bytes = read_file_bytes(bencode_path).unwrap();

    println!(
        "Bencode (.torrent) result:\n{:#?}",
        bencode::parse(file_bytes)
    )
}

/// Get bytes from a given [PathBuf] or return a standard [std::io::Error]
fn read_file_bytes(file: PathBuf) -> Result<Vec<u8>, std::io::Error> {
    let mut file = File::open(file)?;
    let mut contents = vec![];

    file.read_to_end(&mut contents)?;

    Ok(contents)
}

fn main() {
    let torrent_arg = Argument::new(
        &['t', 'f'],
        &["torrent", "file"],
        Some("A .torrent file to download"),
        DataType::File,
    )
    .unwrap();

    let bencode_arg = Argument::new(
        &['b'],
        &["bencode"],
        Some("Parses a .torrent file and displays the resulting AST"),
        DataType::File,
    )
    .unwrap();

    let args = &[torrent_arg.clone(), bencode_arg.clone()]; // torrent_arg + bencode_arg

    let cli = CLIMake::new(
        args,
        Some("torro_climake, an example unfinished torrent client"),
        None,
    )
    .unwrap();

    let mut torrent_arg_buf = None;
    let mut bencode_arg_buf = None;

    for used_arg in cli.parse() {
        if used_arg.argument == torrent_arg {
            torrent_arg_buf = Some(used_arg.passed_data)
        } else if used_arg.argument == bencode_arg {
            bencode_arg_buf = Some(used_arg.passed_data)
        }
    }

    match torrent_arg_buf {
        Some(torrent_arg_data) => match torrent_arg_data {
            PassedData::File(files) => do_torrent(files[0].clone()),
            _ => panic!(),
        },
        None => (), // skip, no `-t` arg passed
    }

    match bencode_arg_buf {
        Some(bencode_arg_data) => match bencode_arg_data {
            PassedData::File(files) => do_bencode(files[0].clone()),
            _ => panic!(),
        },
        None => (), // skip, no `-b` arg passed
    }
}
