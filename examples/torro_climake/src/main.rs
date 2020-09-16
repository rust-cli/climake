use climake::{Argument, CLIMake, DataType, PassedData};
use torro::torrent::Torrent;

fn main() {
    let torrent_arg = Argument::new(
        &['t', 'f'],
        &["torrent", "file"],
        Some("A .torrent file to download"),
        DataType::File,
    )
    .unwrap();

    let cli = CLIMake::new(
        &[torrent_arg.clone()],
        Some("OTorrent, a micro torrent client"),
        None,
    )
    .unwrap();

    let mut torrent_arg_buf = None;

    for used_arg in cli.parse() {
        if used_arg.argument == torrent_arg {
            torrent_arg_buf = Some(used_arg.passed_data)
        }
    }

    match torrent_arg_buf {
        Some(torrent_arg_data) => match torrent_arg_data {
            PassedData::File(file) => println!(
                "Torrent creation result: {:?}",
                Torrent::from_file(file[0].clone())
            ),
            _ => panic!(), // should never happen unless climake screws up
        },
        None => panic!("OTorrent has not implemented this error yet!"),
    }
}
