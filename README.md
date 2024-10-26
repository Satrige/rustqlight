# RustQLite

## Overview
`RustQLite` is an educational project aimed at exploring how a database works under the hood, written entirely in Rust.
This project was inspired by [this tutorial](https://cstack.github.io/db_tutorial/) on writing SQLite in C, but re-imagined using Rust.
Through this project, I'm delving into both Rust programming and database internals, including B+ trees, Write-Ahead Logging (WAL), and the RAFT consensus algorithm.

## Features
- [x] B+ tree implementation for efficient key-value storage
- [ ] Write-Ahead Logging (WAL) for durability
- [ ] RAFT for distributed consistency

## Getting Started
To build the project, simply run:

```bash
cargo build
```

## Links
1. [SQLLIte from scratch](https://cstack.github.io/db_tutorial/)
2. [SQLLite](https://github.com/sqlite/sqlite)

## License
This project is licensed under the MIT License.

