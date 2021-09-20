## Compiling a Rust program

`rustc <source_code>`

## Running a Rust program

`./<compiler_output>`

## Cargo 
Its `npm` but for Rust
`Cargo.toml` (Tom’s Obvious, Minimal Language) is your `package.json` equivalent. `Cargo.lock` is your `package-lock.json` equivalent

## Cargo commands
- `cargo --version` returns cargo version
- `cargo new project_name` starts a new project template (`npm init` equivalent)
- `cargo build` compiles your source code and places its output into `target/debug/compiler_output` path
- `cargo run` compiles your source code and also runs it
- `cargo check` makes sure your code compiles without it actually compiling (much faster than `cargo build` for checkign purposes)
- `cargo build --release` creates an executable in `target/release` path and applies optimizations within the compiling process (runs much slower though).

## Notes

- Rust file names are separated by underscore
- The `main` function is where it all begins
- Declaring a function looks like `fn main() { }`
- Calling a function followed by `!` (`println!(" ")`) means youre actually calling a macro instead