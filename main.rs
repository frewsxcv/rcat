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
        .map(|a| path::PathBuf::from(a));
    for path in iter {
        let file = match open_file(&path) {
            Ok(f) => f,
            Err(e) => {
                print_error(e);
                continue;
            }
        };
        let buf_reader = io::BufReader::new(file);
        for byte in buf_reader.bytes() {
            write!(io::stdout(), "\\x").expect("could not write to stdout");
            let mut byte_hex = format!("{:x}", byte.expect("could not read byte"));
            if byte_hex.len() == 1 {
                byte_hex.insert(0, '0');
            }
            write!(io::stdout(), "{}", byte_hex).expect("could not write to stdout");
        }
    }
    // TODO: don't print newline if nothing was written
    write!(io::stdout(), "\n").expect("could not write to stdout");

    // TODO: return failing status code if error
}
