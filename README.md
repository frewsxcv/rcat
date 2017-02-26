# hexcat

Utility to output a file's contents in Rust's hexadecimal notation.

## Install

```
cargo install hexcat
```

## Usage 

```
$ cat foo.txt
Hello world!
$ hexcat foo.txt 
Hello\x20world\x21\x0a
```
