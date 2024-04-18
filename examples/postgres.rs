#[cfg(feature = "postgres")]
use keyv::{adapter::postgres::PostgresStoreBuilder, Keyv};

/* To test, run a postgres docker image
docker run --name keyv-postgres-test \
-e POSTGRES_USER=postgres \
-e POSTGRES_PASSWORD=postgres \
-e POSTGRES_DB=keyv_test \
-p 5432:5432 \
-d postgres:latest
*/
#[cfg(feature = "postgres")]
#[tokio::main]
async fn main() {
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

#[cfg(not(feature = "postgres"))]
fn main() {
    println!("This example requires the 'postgres' feature to be enabled.");
    println!("Please run the command as follows:");
    println!("cargo run --example postgres --features postggres");
}
