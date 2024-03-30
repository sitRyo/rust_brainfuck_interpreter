# rust_brainfuck_interpreter
brainfuck interpreter written in Rust.

## Usage
### Build And Run
```bash
$ cd interpreter
$ cargo build
$ cargo run
```

### Options
#### -d, --debug
Execute in step mode while displaying memory and cursor position.
```
$ cargo run -- -d
+++++++[>+++++++<-]>-
                ^
7|7
runtime memory cursor: 1
```
#### -h, --help
Show help.

## References
- [Introduction - The Cargo Book](https://doc.rust-lang.org/cargo/index.html)
- [Brainfuck - Wikibooks](https://ja.wikibooks.org/wiki/Brainfuck)
- [The Rust Programming Language 日本語版 - The Rust Programming Language 日本語版](https://doc.rust-jp.rs/book-ja/)
