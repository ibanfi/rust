use std::io::Read;
use serde_json::{Value};

#[test]
fn get_user() {
    let mut resp = String::new();
    reqwest::get("http://localhost:3020/api/v1/users")
        .unwrap()
        .read_to_string(&mut resp)
        .unwrap();
    println!("Resp: {}", resp);
    // It is not empty
    assert!(!resp.is_empty());

    let v: Value = serde_json::from_str(&resp).unwrap(); 
    // It is an array
    assert!(v.is_array());
    // The array size is 10
    assert_eq!(v.as_array().unwrap().len(), 10);

    // Checking the first element only
    // Has needed attributes
    assert_ne!(v[0].get("id"), None);
    assert_ne!(v[0].get("name"), None);
    assert_ne!(v[0].get("email"), None);
    // Missing the others
    assert_eq!(v[0].as_object().unwrap().len(), 3);
}

#[test]
fn post_user_01() {
    let msg = r#"{ "name": "MF", "email": "bad" }"#;
    let client = reqwest::Client::new();
    let resp = client.post("http://localhost:3020/api/v1/users")
        .header("Content-Type", "application/json")
        .body(msg)
        .send()
        .unwrap();
    // Check whether the HTTP Code is 422
    assert_eq!(resp.status(), reqwest::StatusCode::UNPROCESSABLE_ENTITY);

}

#[test]
fn post_user_02() {
    let msg = r#"{"name": "Martin Fowler", "email": "martin@martinfowler.com" }"#;
    let client = reqwest::Client::new();
    let resp = client.post("http://localhost:3020/api/v1/users")
        .header("Content-Type", "application/json")
        .body(msg)
        .send()
        .unwrap();
    // Check whether the HTTP Code is 200
    assert_eq!(resp.status(), reqwest::StatusCode::OK);
}

