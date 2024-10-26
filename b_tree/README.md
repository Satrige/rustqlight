# `b_tree` Crate

The `b_tree` crate provides a [B+ tree](https://en.wikipedia.org/wiki/B%2B_tree) data structure implementation as part of the `RustQLite` project, an educational exploration of database internals using Rust.

Currently, keys must implement the `Ord + Copy` traits, and values must implement the `Clone` trait.

## Features
- [x] **Insertion**: Supports adding data into the B+ tree structure
- [x] **Search**: Allows for fast, key-based search functionality
- [x] **Zero-Copy Approach**: Data is only cloned upon initial insertion into nodes, optimizing memory usage and minimizing unnecessary copies
- [ ] **Deletion**: The ability to delete nodes or data by key.
- [ ] **Range Searches**: Allows retrieval of a range of values by key.

## Running Tests

To run tests for this crate, use:

```bash
cargo test
```

## License

The main `RustQLite` project is under the MIT License, and this crate can follow the same licensing terms if applicable. Check the root project’s LICENSE file for more details.

