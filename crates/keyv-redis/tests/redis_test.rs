use keyv::Keyv;
use keyv_redis::RedisStoreBuilder; // Assuming a similar naming pattern as PostgresStoreBuilder.

#[tokio::test]
async fn test_keyv_redis() {
    // Construct a RedisStore via its builder
    let store = RedisStoreBuilder::new()
        .uri("redis://localhost:6379") // Adjust based on your Redis setup.
        .default_ttl(3600) // Optional: Sets a default TTL for keys.
        .build()
        .await
        .unwrap();

    // Initialize Keyv with the Redis store
    let keyv = Keyv::try_new(store).await.unwrap();

    // Perform set and get operations, similar to the Postgres example
    keyv.set("number", 42).await.unwrap();
    keyv.set("number", 10).await.unwrap();
    keyv.set("array", vec!["hola", "test"]).await.unwrap();
    keyv.set("string", "life long").await.unwrap();

    // Assert get operations
    let number: i32 = keyv
        .get("number")
        .await
        .unwrap()
        .map(|v| serde_json::from_value(v).unwrap())
        .unwrap();
    assert_eq!(number, 10);

    let string: String = keyv
        .get("string")
        .await
        .unwrap()
        .map(|v| serde_json::from_value(v).unwrap())
        .unwrap();
    assert_eq!(string, "life long");

    let array: Vec<String> = keyv
        .get("array")
        .await
        .unwrap()
        .map(|v| serde_json::from_value(v).unwrap())
        .unwrap();
    assert_eq!(array, vec!["hola".to_string(), "test".to_string()]);
}
