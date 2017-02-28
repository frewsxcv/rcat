#[macro_use]
extern crate clap;

use std::{ascii, error, fs, io, path};
use std::io::{Read, Write};

static PROGRAM_NAME: &'static str = "rcat";
static REPO_URL: &'static str = "https://github.com/frewsxcv/rcat";

fn open_file(path: &path::Path) -> Result<fs::File, Box<error::Error>> {
    Ok(path)
        .and_then(|path| if path.is_file() {
            Ok(path)
        } else {
            Err("not a file".into())
        })
        .and_then(|p| fs::File::open(p).map_err(|e| e.into()))
}

fn print_error(error: Box<error::Error>) {
    writeln!(io::stderr(), "{}: {}", PROGRAM_NAME, error.description())
        .expect("could not write to stderr");
}

fn print_byte<W: Write>(byte: u8, writer: &mut W) {
    for char_ in ascii::escape_default(byte) {
        writer.write_all(&[char_]).expect("could not write to stdout")
    }
}

/// Return if at least one byte was printed
fn print_bytes_from_reader<R: Read, W: Write>(reader: R, writer: &mut W) -> bool {
    let mut was_byte_printed = false;
    for byte in io::BufReader::new(reader)
        .bytes()
        .map(|b| b.expect("could not read byte from reader")) {
        was_byte_printed = true;
        print_byte(byte, writer)
    }
    was_byte_printed
}

static FILES_ARG_NAME: &'static str = "file";
static QUOTE_ARG_NAME: &'static str = "quote";

fn main() {
    let matches = clap::App::new(PROGRAM_NAME)
        .version(crate_version!())
        .about(REPO_URL)
        .arg(clap::Arg::with_name(FILES_ARG_NAME).multiple(true))
        .arg(clap::Arg::with_name(QUOTE_ARG_NAME)
            .long("quote")
            .help("Quote output using Rust's byte slice literal syntax")
        )
        .get_matches();

    let mut stdout = io::BufWriter::new(io::stdout());

    let should_quote = matches.values_of(QUOTE_ARG_NAME).is_some();

    if should_quote {
        stdout.write_all(&[b'b', b'"']).expect("could not write to stdout");
    }

    let file_values = match matches.values_of(FILES_ARG_NAME) {
        Some(f) => f,
        None => {
            let was_byte_printed = print_bytes_from_reader(io::stdin(), &mut stdout);
            if should_quote {
                stdout.write_all(&[b'"']).expect("could not write to stdout");
            }
            if was_byte_printed || should_quote {
                stdout.write_all(&[b'\n']).expect("could not write to stdout");
            }
            return;
        }
    };

    let iter = file_values.map(|m| m.as_ref())
        .map(|p| open_file(p));

    let mut was_byte_printed = false;
    for result in iter {
        match result {
            Ok(f) => was_byte_printed |= print_bytes_from_reader(f, &mut stdout),
            Err(e) => print_error(e),
        }
    }
    if should_quote {
        stdout.write_all(&[b'"']).expect("could not write to stdout");
    }
    if was_byte_printed || should_quote {
        stdout.write_all(&[b'\n']).expect("could not write to stdout");
    }

    // TODO: return failing status code if error
}
