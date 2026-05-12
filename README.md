# crust

A hobby project, a tiny C compiler written in Rust that emits assembly.

## Why

- Learning Rust, it's a new language for me.
- Learning how compilers work (lexing, parsing, codegen), I've never built one before.
- Learning how ARM64 assembly works, I'm only somewhat familiar with x86 (32-bit) and AVR (8-bit microcontrollers).
- Learning how native MacOS development works (e.g. syscalls), I've only done Win32 development before.
- Aiming for something concrete, take a small subset of C and turn it into assembly.

Inspired by [Tsoding's compiler video](https://www.youtube.com/watch?v=Yi6NxMxCFY8).

## No Vibes 👻

The implementation is written by hand (old school 🤓). AI is only used to ask questions about Rust, language features, patterns, idiomatic style, etc.

## Example

The compiler takes a `.c` file as input and writes assembly to stdout.

```sh
cargo run -q examples/hello.c > examples/hello.s
cc -o examples/hello examples/hello.s
./examples/hello
echo $?
```

## Status

| Stage             | Status      |
| ----------------- | ----------- |
| Lexer             | Working     |
| Parser            | In progress |
| Semantic analysis | Not started |
| Codegen           | In progress |

### Supported today

| Feature                                     | Supported                                                           |
| ------------------------------------------- | ------------------------------------------------------------------- |
| `int` type                                  | Yes                                                                 |
| Function definition (`int main()`, no args) | Yes                                                                 |
| `return <int-literal>;`                     | Yes                                                                 |
| `printf("<string-literal>");`               | Hardcoded to a `write` syscall to stdout, no format-string handling |
| `int <name> = <int-literal>;`               | Yes, stored on the stack                                            |
| `int <name> = <var>;`                       | Yes, copied from another stack slot                                 |
| Other types                                 | No                                                                  |
| User-defined functions / function calls     | No                                                                  |
| Expressions                                 | No, RHS of an initializer is either a literal or a single variable  |
| Control flow                                | No                                                                  |
