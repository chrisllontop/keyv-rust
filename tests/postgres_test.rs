#[cfg(feature = "postgres")]
use keyv::{adapter::postgres::PostgresStoreBuilder, Keyv};

/* To run the test run the docker mysql
docker run --name keyv-postgres-test \
-e POSTGRES_USER=postgres \
-e POSTGRES_PASSWORD=postgres \
-e POSTGRES_DB=keyv_test \
-p 5432:5432 \
-d postgres:latest
*/
#[cfg(feature = "postgres")]
#[tokio::test]
async fn test_keyv_postgres() {
    let store = PostgresStoreBuilder::new()
        .uri("postgresql://postgres:postgres@localhost:5432")
        .schema("cache")
        .build()
        .await
        .unwrap();

    let keyv = Keyv::try_new(store).await.unwrap();
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
}
