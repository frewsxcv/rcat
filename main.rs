use std::{env, fs, io, path};
use std::io::{Read, Write};

fn main() {
    let mut args = env::args_os();
    let name = args.next().unwrap();
    let iter = args.map(|a| path::PathBuf::from(a));
    for path in iter {
        if !path.is_file() {
            writeln!(io::stderr(),
                     "{}: {}: not a file",
                     name.to_string_lossy(),
                     path.to_string_lossy())
                .expect("could not write to stderr");
            continue;
        }
        let file = match fs::File::open(&path) {
            Ok(f) => f,
            Err(e) => {
                writeln!(io::stderr(),
                         "{}: {}: {}",
                         name.to_string_lossy(),
                         path.to_string_lossy(),
                         e)
                    .expect("could not write to stderr");
                continue;
            }
        };
        let buf_reader = io::BufReader::new(file);
        for byte in buf_reader.bytes() {
            write!(io::stdout(), "\\x{:x}", byte.unwrap()).expect("could not write to stdout");
        }
    }
    write!(io::stdout(), "\n").expect("could not write to stdout");
}
