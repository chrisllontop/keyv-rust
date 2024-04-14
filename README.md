<h1 align="center">
	<img width="250" src="https://jaredwray.com/images/keyv.svg" alt="keyv">
	<br>
	<br>
</h1>

> Simple key-value storage with support for multiple backends
> Keyv provides a consistent interface for key-value storage across multiple backends via storage adapters. It supports TTL based expiry, making it suitable as a cache or a persistent key-value store.

## Usage

Install Keyv.

```
cargo add  keyv
```

By default everything is stored in memory, you can optionally also install a storage adapter.

- Inmemory default
  ```rust
  let keyv = Keyv::default();
  ```
- Postgres

  ```rust
     let store = PostgresStoreBuilder::new()
         .uri("postgresql://postgres:postgres@localhost:5432")
         .table_name("custom_table_name")
         .build()
         .await.unwrap();

     let keyv = Keyv::try_new(store).await.unwrap();
  ```

- Redis

  ```rust
    let store = RedisStoreBuilder::new()
        .uri("redis://localhost:6379")
        .default_ttl(3600)
        .build()
        .await
        .unwrap();

     let keyv = Keyv::try_new(store).await.unwrap();
  ```
