#[macro_use]
extern crate clap;

use std::{ascii, error, fs, io, path};
use std::io::{Read, Write};

fn is_file_at_path(path: &path::Path) -> Result<&path::Path, Box<error::Error>> {
    if path.is_file() {
        Ok(path)
    } else {
        Err("not a file".into())
    }
}

fn open_file(path: &path::Path) -> Result<fs::File, Box<error::Error>> {
    Ok(path)
        .and_then(is_file_at_path)
        .and_then(|p| fs::File::open(p).map_err(|e| e.into()))
}

fn print_error(error: Box<error::Error>) {
    writeln!(io::stderr(), "hexcat: {}", error.description()).expect("could not write to stderr");
}

fn print_byte(byte: u8) {
    for char_ in ascii::escape_default(byte).map(|b| b as char) {
        write!(io::stdout(), "{}", char_)
            .expect("could not write to stdout")
    }
}

fn print_bytes_from_reader<R: Read>(reader: R) {
    for byte in io::BufReader::new(reader)
        .bytes()
        .map(|b| b.expect("could not read byte from reader"))
    {
        print_byte(byte)
    }
}

fn main() {
    let matches = clap::App::new("hexcat")
        .version(crate_version!())
        .about("https://github.com/frewsxcv/hexcat")
        .arg(clap::Arg::with_name("file")
            .multiple(true))
        .get_matches();

    // https://github.com/kbknapp/clap-rs/pull/877

    let file_values = matches.values_of("file");

    let file_values = match file_values {
        Some(f) => f,
        None => {
            print_bytes_from_reader(io::stdin());
            return;
        },
    };

    let iter = file_values
        .map(|m| m.as_ref())
        .map(|p| open_file(p));

    for result in iter {
        match result {
            Ok(f) => print_bytes_from_reader(f),
            Err(e) => print_error(e),
        }
    }
    // TODO: don't print newline if nothing was written
    write!(io::stdout(), "\n").expect("could not write to stdout");

    // TODO: return failing status code if error
}
