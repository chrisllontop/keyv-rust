use keyv::Keyv;

#[tokio::main]
async fn main() {
    let keyv = Keyv::default();
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

    match keyv.remove_many(&["number", "string"]).await {
        Ok(_) => {}
        Err(_) => println!("Failed to remove keys"),
    }
}
