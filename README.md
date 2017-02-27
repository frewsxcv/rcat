# rcat

Utility to output file contents using Rust's byte slice notation.

## Install

```
cargo install rcat
```

## Usage

```
$ cat foo.txt
Hello world! 👋🌎
$ rcat foo.txt
Hello world! \xf0\x9f\x91\x8b\xf0\x9f\x8c\x8e\n
$ rcat --quoted foo.txt
b"Hello world! \xf0\x9f\x91\x8b\xf0\x9f\x8c\x8e\n"
```
