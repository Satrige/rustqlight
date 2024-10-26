# `database` Crate

The `database` crate forms the core of the `RustQLite` project, an educational database inspired by [this SQLite tutorial](https://cstack.github.io/db_tutorial/). This crate provides foundational database functionality, currently supporting a predefined table structure and basic data persistence.

## Features

- **Predefined Table Structure**: The database currently supports a single, predefined table with the following structure:
  - `id`: `u32`
  - `email`: `String` (max length 256)
  - `user_name`: `String` (max length 256)
  
- **Persistence**: 
  - **Dump and Load**: Dumps the current database state into a file and reads this file on startup to restore data.

- **Interactive REPL**:
  - **Insert Records**: Insert a new record with `insert test@test.test test_user`.
  - **Select by Primary Key**: Retrieve records by primary key with `select 1`.

## Further Improvements

1. **Add Tests**
2. **Dynamic Table Structure**: Enable user-defined tables to expand functionality.
3. **Asynchronous Handling**: Integrate with Tokio to handle multiple requests concurrently.

## License

The main `RustQLite` project is licensed under the MIT License, and this crate follows the same terms. Check the root project’s LICENSE file for more information.
