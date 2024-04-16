#[cfg(feature = "redis")]
use keyv::{adapter::redis::RedisStoreBuilder, Keyv};

/* To test, run a redis docker image
docker run --name keyv-redis-test -p 6379:6379 -d redis:latest
*/
#[cfg(feature = "redis")]
#[tokio::test]
async fn test_keyv_redis() {
    let store = RedisStoreBuilder::new()
        .uri("redis://localhost:6379")
        .default_ttl(3600)
        .build()
        .await
        .unwrap();

    let keyv = Keyv::try_new(store).await.unwrap();

    keyv.set("number", 42).await.unwrap();
    keyv.set("number", 10).await.unwrap();
    keyv.set("array", vec!["hola", "test"]).await.unwrap();
    keyv.set("string", "life long").await.unwrap();

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
