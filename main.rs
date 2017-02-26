#[macro_use]
extern crate clap;

use std::{error, fs, io, path};
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
    write!(io::stdout(), "\\x{0:01$x}", byte, 2)
        .expect("could not write to stdout");
}

fn print_file(file: fs::File) {
    for byte in io::BufReader::new(file)
        .bytes()
        .map(|b| b.expect("could not read byte from file"))
    {
        print_byte(byte)
    }
}

fn main() {
    let matches = clap::App::new("hexcat")
        .version(crate_version!())
        .about("https://github.com/frewsxcv/hexcat")
        .arg(clap::Arg::with_name("file")
            .multiple(true)
            .required(true))
        .get_matches();

    let iter = matches.values_of("file")
        .expect("did not receive file names")
        .map(|p| open_file(p.as_ref()));

    for result in iter {
        match result {
            Ok(f) => print_file(f),
            Err(e) => print_error(e),
        }
    }
    // TODO: don't print newline if nothing was written
    write!(io::stdout(), "\n").expect("could not write to stdout");

    // TODO: return failing status code if error
}
