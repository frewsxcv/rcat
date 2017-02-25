use std::{env, error, fs, io, path};
use std::io::{Read, Write};

fn open_file(path: &path::Path) -> Result<fs::File, Box<error::Error>> {
    fs::File::open(&path)
        .map_err(|e| format!("{}: {}", path.to_string_lossy(), e).into())
}

fn print_error(error: Box<error::Error>) {
    writeln!(io::stderr(), "hexcat: {}", error).expect("could not write to stderr");
}

fn main() {
    let mut args = env::args_os();
    let name = args.next().expect("failed to retrieve arguments");
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
        let file = match open_file(&path) {
            Ok(f) => f,
            Err(e) => {
                print_error(e);
                continue;
            }
        };
        let buf_reader = io::BufReader::new(file);
        for byte in buf_reader.bytes() {
            write!(io::stdout(), "\\x")
                .expect("could not write to stdout");
            let mut byte_hex = format!("{:x}", byte.expect("could not read byte"));
            if byte_hex.len() == 1 {
                byte_hex.insert(0, '0');
            }
            write!(io::stdout(), "{}", byte_hex)
                .expect("could not write to stdout");
        }
    }
    // TODO: don't print newline if nothing was written
    write!(io::stdout(), "\n").expect("could not write to stdout");

    // TODO: return failing status code if error
}
