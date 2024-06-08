# Keyv Rust

Simple key-value storage with support for multiple backends.

[![Crates.io](https://img.shields.io/crates/v/keyv)](https://crates.io/crates/keyv)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/chrisllontop/keyv-rust/ci.yml)
![GitHub Repo stars](https://img.shields.io/github/stars/chrisllontop/keyv-rust)
![Crates.io Total Downloads](https://img.shields.io/crates/d/keyv)

> Simple key-value storage with support for multiple backends
> Keyv provides a consistent interface for key-value storage across multiple backends via storage adapters. It supports
> TTL based expiry, making it suitable as a cache or a persistent key-value store.

## Usage

### Instalation

```bash
cargo add keyv
```

### Store Adapters

Keyv supports multiple store adapters, you can enable them by specifying the feature flag.

- **full**: Enables all available adapters.
- **[redis](https://github.com/chrisllontop/keyv-rust/tree/main/src/store/adapter/redis)**: Redis store adapter.
- **[postgres](https://github.com/chrisllontop/keyv-rust/tree/main/src/store/adapter/postgres)**: PostgreSQL store
  adapter.
- **[mysql](https://github.com/chrisllontop/keyv-rust/tree/main/src/store/adapter/mysql)**: MySQL store adapter.
- **[mongodb](https://github.com/chrisllontop/keyv-rust/tree/main/src/store/adapter/mongodb)**: MongoDB store adapter.
- **[sqlite](https://github.com/chrisllontop/keyv-rust/tree/main/src/store/adapter/sqlite)**: SQLite store adapter.

```bash
cargo add keyv --features <store>
```

### Initialization

By default, everything is stored in memory, you can optionally also install a storage adapter.

- Inmemory (default)
  ```rust
  let keyv = Keyv::default();
  ```
- Store Adapters examples

    - [Postgres](https://github.com/chrisllontop/keyv-rust/tree/main/examples/postgres.rs)
    - [Redis](https://github.com/chrisllontop/keyv-rust/tree/main/examples/redis.rs)
    - [Mongodb](https://github.com/chrisllontop/keyv-rust/tree/main/examples/mongodb.rs)
    - [Sqlite](https://github.com/chrisllontop/keyv-rust/tree/main/examples/sqlite.rs)
    - [MySQL](https://github.com/chrisllontop/keyv-rust/tree/main/examples/mysql.rs)

### Interacting with Store

```rust

use keyv::Keyv;

#[tokio::main]
async fn main() {
    let keyv = Keyv::default();
    keyv.set("number", 42).await.unwrap();
    keyv.set("number", 10).await.unwrap();
    keyv.set("array", vec!["hola", "test"]).await.unwrap();
    keyv.set("string", "life long").await.unwrap();

    match keyv.get("number").await.unwrap() {
        Some(number) => {
            let number: i32 = serde_json::from_value(number).unwrap();
            assert_eq!(number, 10);
        }
        None => assert!(false),
    }

    match keyv.get("string").await.unwrap() {
        Some(string) => {
            let string: String = serde_json::from_value(string).unwrap();
            assert_eq!(string, "life long");
        }
        None => assert!(false),
    }

    match keyv.get("array").await.unwrap() {
        Some(array) => {
            let array: Vec<String> = serde_json::from_value(array).unwrap();
            assert_eq!(array, vec!["hola".to_string(), "test".to_string()])
        }
        None => assert!(false),
    }

    match keyv.remove_many(&["number", "string"]).await {
        Ok(_) => {}
        Err(_) => assert!(false),
    }
}
```
