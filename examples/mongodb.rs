/* To test, run a MongoDB docker image
docker run --name keyv-mongo-test \
-e MONGO_INITDB_ROOT_USERNAME=mongo \
-e MONGO_INITDB_ROOT_PASSWORD=mongo \
-p 27017:27017 \
-d mongo:latest
*/

#[cfg(feature = "mongo")]
use keyv::{adapter::mongodb::MongoStoreBuilder, Keyv};

#[cfg(feature = "mongo")]
#[tokio::main]
async fn main() {
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
            println!("number: {}", number);
        }
        None => println!("number not found"),
    }

    match keyv.get("string").await.unwrap() {
        Some(string) => {
            let string: String = serde_json::from_value(string).unwrap();
            println!("string: {}", string);
        }
        None => println!("string not found"),
    }

    match keyv.get("array").await.unwrap() {
        Some(array) => {
            let array: Vec<String> = serde_json::from_value(array).unwrap();
            println!("array: {:?}", array);
        }
        None => println!("array not found"),
    }
}

#[cfg(not(feature = "mongo"))]
fn main() {
    println!("This example requires the 'mongo' feature to be enabled.");
    println!("Please run the command as follows:");
    println!("cargo run --example mongodb --features mongo");
}
