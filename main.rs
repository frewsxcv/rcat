use std::{env, fs, io, path};
use std::io::{Read, Write};

fn main() {
    let mut args = env::args_os();
    let name = args.next().unwrap();
    let iter = args
        .map(|a| path::PathBuf::from(a));
    for path in iter {
        if !path.is_file() {
            continue;
        }
        let file = io::BufReader::new(fs::File::open(path).unwrap());
        println!("filename: ");
        for byte in file.bytes() {
            write!(io::stdout(), "\\x{:x}", byte.unwrap());
        }
        write!(io::stdout(), "\n");

        //println!("{:x}", args.as_bytes());
    }
}
