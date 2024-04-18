#[cfg(feature = "sqlite")]
use keyv::{adapter::sqlite::SqliteStoreBuilder, Keyv};

#[cfg(feature = "sqlite")]
#[tokio::main]
async fn main() {
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
            println!("number: {}", number);
        }
        None => println!("number not found"),
    }

    match keyv.get("string").await.unwrap() {
        Some(string) => {
            let string: String = serde_json::from_value(string).unwrap(); // Deserialize
            println!("string: {}", string);
        }
        None => println!("string not found"),
    }

    match keyv.get("array").await.unwrap() {
        Some(array) => {
            let array: Vec<String> = serde_json::from_value(array).unwrap(); // Deserialize
            println!("array: {:?}", array);
        }
        None => println!("array not found"),
    }
}

#[cfg(not(feature = "sqlite"))]
fn main() {
    println!("This example requires the 'sqlite' feature to be enabled.");
    println!("Please run the command as follows:");
    println!("cargo run --example sqlite --features sqlite");
}
