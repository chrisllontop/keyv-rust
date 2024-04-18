#[cfg(feature = "sqlite")]
use keyv::{adapter::sqlite::SqliteStoreBuilder, Keyv};

#[cfg(feature = "sqlite")]
#[tokio::test]
async fn test_keyv_sqlite() {
    // Instead of using a URI, we will use SQLite's in-memory database for testing purposes.
    let store = SqliteStoreBuilder::new()
        .uri("sqlite::memory:") // In-memory database, good for tests
        .table_name("cache") // Optional if you want to specify a different table name
        .build()
        .await
        .unwrap();

    let keyv = Keyv::try_new(store).await.unwrap();

    // Setting various types of data
    keyv.set("number", 42).await.unwrap(); // set initial value
    keyv.set("number", 10).await.unwrap(); // update value
    keyv.set("array", vec!["hello", "world"]).await.unwrap();
    keyv.set("string", "test value").await.unwrap();

    // Test retrieving and validating data
    match keyv.get("number").await.unwrap() {
        Some(number) => {
            let number: i32 = serde_json::from_value(number).unwrap(); // Deserialize
            assert_eq!(number, 10);
        }
        None => assert!(false, "Expected data not found"),
    }

    match keyv.get("string").await.unwrap() {
        Some(string) => {
            let string: String = serde_json::from_value(string).unwrap(); // Deserialize
            assert_eq!(string, "test value");
        }
        None => assert!(false, "Expected data not found"),
    }

    match keyv.get("array").await.unwrap() {
        Some(array) => {
            let array: Vec<String> = serde_json::from_value(array).unwrap(); // Deserialize
            assert_eq!(array, vec!["hello".to_string(), "world".to_string()]);
        }
        None => assert!(false, "Expected data not found"),
    }
}
