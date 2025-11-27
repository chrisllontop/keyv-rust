#[cfg(feature = "mongo")]
use keyv::{adapter::mongodb::MongoStoreBuilder, Keyv};

/* To test, run a MongoDB docker image
docker run --name keyv-mongo-test \
-e MONGO_INITDB_ROOT_USERNAME=mongo \
-e MONGO_INITDB_ROOT_PASSWORD=mongo \
-p 27017:27017 \
-d mongo:latest
*/
#[cfg(feature = "mongo")]
#[tokio::test]
async fn test_keyv_mongo() {
    let store = MongoStoreBuilder::new()
        .uri("mongodb://mongo:mongo@localhost:27017")
        .database_name("keyv_test")
        .collection_name("cache")
        .build()
        .await
        .unwrap();

    let keyv = Keyv::try_new(store).await.unwrap();
    keyv.set("number", 42).await.unwrap();
    keyv.set("number", 10).await.unwrap();
    keyv.set("array", vec!["hello", "test"]).await.unwrap();
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
            assert_eq!(array, vec!["hello".to_string(), "test".to_string()])
        }
        None => assert!(false),
    }
    // Test remove
    keyv.remove("number").await.unwrap();
    match keyv.get("number").await.unwrap() {
        Some(_) => assert!(false, "number should have been removed"),
        None => {}
    }

    // Test remove_many
    keyv.set("key0", "value0").await.unwrap();
    keyv.remove_many(&["string", "array"]).await.unwrap();
    match keyv.get("string").await.unwrap() {
        Some(_) => assert!(false, "string should have been removed"),
        None => {}
    }
    match keyv.get("array").await.unwrap() {
        Some(_) => assert!(false, "array should have been removed"),
        None => {}
    }
    match keyv.get("key0").await.unwrap() {
        Some(_) => {}
        None => assert!(false, "key0 shouldn't have been removed"),
    }
    // Test clear
    keyv.set("key1", "value1").await.unwrap();
    keyv.set("key2", "value2").await.unwrap();
    keyv.clear().await.unwrap();
    match keyv.get("key0").await.unwrap() {
        Some(_) => assert!(false, "key0 should have been removed after clear"),
        None => {}
    }
    match keyv.get("key1").await.unwrap() {
        Some(_) => assert!(false, "key1 should have been removed after clear"),
        None => {}
    }
    match keyv.get("key2").await.unwrap() {
        Some(_) => assert!(false, "key2 should have been removed after clear"),
        None => {}
    }
}
