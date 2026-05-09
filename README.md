# crust

A hobby project, a tiny C compiler written in Rust that emits assembly.

## Why

- Learning Rust, it's a new language for me.
- Learning how compilers work (lexing, parsing, codegen), I've never built one before.
- Aiming for something concrete, take a small subset of C and turn it into assembly.

Inspired by [Tsoding's compiler video](https://www.youtube.com/watch?v=Yi6NxMxCFY8).

## No Vibes 👻

The implementation is written by hand (old school 🤓). AI is only used to ask questions about Rust, language features, patterns, idiomatic style, etc.

## Running

```sh
cargo run
```

## Status

- [x] Lexing
- [ ] Parsing
- [ ] Semantic analysis (maybe)
- [ ] Code generation
