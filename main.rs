#[macro_use]
extern crate clap;

use std::{ascii, error, fs, io, path};
use std::io::{Read, Write};

fn open_file(path: &path::Path) -> Result<fs::File, Box<error::Error>> {
    Ok(path)
        .and_then(|path| if path.is_file () {
            Ok(path)
        } else {
            Err("not a file".into())
        })
        .and_then(|p| fs::File::open(p).map_err(|e| e.into()))
}

fn print_error(error: Box<error::Error>) {
    writeln!(io::stderr(), "hexcat: {}", error.description()).expect("could not write to stderr");
}

fn print_byte<W: Write>(byte: u8, writer: &mut W) {
    for char_ in ascii::escape_default(byte) {
        writer.write_all(&[char_]).expect("could not write to stdout")
    }
}

fn print_bytes_from_reader<R: Read, W: Write>(reader: R, writer: &mut W) {
    for byte in io::BufReader::new(reader)
        .bytes()
        .map(|b| b.expect("could not read byte from reader")) {
        print_byte(byte, writer)
    }
}

fn main() {
    let matches = clap::App::new("hexcat")
        .version(crate_version!())
        .about("https://github.com/frewsxcv/hexcat")
        .arg(clap::Arg::with_name("file").multiple(true))
        .get_matches();

    // https://github.com/kbknapp/clap-rs/pull/877

    let file_values = matches.values_of("file");

    let mut stdout = io::BufWriter::new(io::stdout());

    let file_values = match file_values {
        Some(f) => f,
        None => {
            print_bytes_from_reader(io::stdin(), &mut stdout);
            return;
        }
    };

    let iter = file_values.map(|m| m.as_ref())
        .map(|p| open_file(p));

    for result in iter {
        match result {
            Ok(f) => print_bytes_from_reader(f, &mut stdout),
            Err(e) => print_error(e),
        }
    }
    // TODO: don't print newline if nothing was written
    stdout.write_all(&[b'\n']).expect("could not write to stdout");

    // TODO: return failing status code if error
}
