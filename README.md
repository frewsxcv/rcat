**⚠️ This repository has moved to <https://tangled.org/rwell.org/rcat> ⚠️**

# rcat

Concatenate and print files in a Rust byte slice compatible format.

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
$ rcat --quote foo.txt
b"Hello world! \xf0\x9f\x91\x8b\xf0\x9f\x8c\x8e\n"
```
