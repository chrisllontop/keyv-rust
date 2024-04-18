#[cfg(feature = "redis")]
use keyv::{adapter::redis::RedisStoreBuilder, Keyv};

/* To test, run a redis docker image
docker run --name keyv-redis-test -p 6379:6379 -d redis:latest
*/
#[cfg(feature = "redis")]
#[tokio::main]
async fn main() {
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
    println!("number: {}", number);

    let string: String = keyv
        .get("string")
        .await
        .unwrap()
        .map(|v| serde_json::from_value(v).unwrap())
        .unwrap();
    println!("string: {}", string);

    let array: Vec<String> = keyv
        .get("array")
        .await
        .unwrap()
        .map(|v| serde_json::from_value(v).unwrap())
        .unwrap();
    println!("array: {:?}", array);
}

#[cfg(not(feature = "redis"))]
fn main() {
    println!("This example requires the 'redis' feature to be enabled.");
    println!("Please run the command as follows:");
    println!("cargo run --example redis --features redis");
}
