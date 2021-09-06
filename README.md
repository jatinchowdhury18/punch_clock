# Punch Clock

This repository contains a little command-line
punch clock application that I wrote as a practice
project while trying to learn Rust.

This code is definitely not production-ready.

Things to fix:
- remove dependencies on local filesystem.
- better report generation.
- separate tracker file for each project?

## Build Instructions

You must already have the [Rust toolchain](https://rust-lang.github.io/rustup/concepts/toolchains.html)
installed on your system.

Clone repository:
```bash
git clone https://github.com/jatinchowdhury18/punch_clock
```

Build:
```bash
cargo build
```

Build with optimizations:
```bash
cargo build --release
```

Install:
```bash
cargo install
```
