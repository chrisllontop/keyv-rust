use keyv::Keyv;

#[tokio::test]
async fn test_keyv() {
    let keyv = Keyv::default();
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

    match keyv.remove_many(&["number", "string"]).await {
        Ok(_) => {}
        Err(_) => assert!(false),
    }
}
