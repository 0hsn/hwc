hwc command
========

`hwc` is an alternative command to *nix `wc` command. This project is don't have compatible output to `wc` command.

> This is an unstable project. DO NOT use this project for production purpose. This project is intended for learning purpose only. This project demonstrates what I have learned so far until CH. 10 of The Rust Book.

##### Help
```bash
USAGE:
    hwc [FLAGS] FILE

FLAGS:
    -b               Prints number of bytes
    -c               Prints number of characters
    -h, --help       Prints help information
    -l               Prints number of lines
    -V, --version    Prints version information
    -w               Prints number of words

ARGS:
    FILE             Filename to check from
```