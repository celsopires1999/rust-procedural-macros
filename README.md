# Rust Macros Study

This repository is based on two publications that I brought together to learn Rust Macros:

- [**Macros in Rust: A tutorial with examples**](https://blog.logrocket.com/macros-in-rust-a-tutorial-with-examples/): covers all kind of macros, but my focus is on procedural macros in special Attribute-like macros

- [**Nine Rules for Creating Procedural Macros in Rust**](https://towardsdatascience.com/nine-rules-for-creating-procedural-macros-in-rust-595aa476a7ff/): this article shows how to create procedural macros that can be debugged and other interesting "rules"

## Recommendation

Visit https://github.com/CarlKCarlK/anyinput repository to see the implementation by Carl Kadie

## How it works

Run unit tests

```bash
cargo test -p printvars-core
```

Run integration tests

```bash
cargo test
```
