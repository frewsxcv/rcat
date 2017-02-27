# rcat

Utility to output file contents using Rust's byte slice notation.

## Install

```
cargo install rcat
```

## Usage

```
$ cat foo.txt
Hello world!
$ rcat foo.txt
Hello\x20world\x21\x0a
```
