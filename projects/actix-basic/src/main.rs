use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io::Read;
use serde_json;
use serde::{Serialize, Deserialize};
use validator_derive::{Validate};
use validator::{Validate};

#[derive(Serialize, Deserialize, Validate, Debug)]
struct User {
    id: i8,  
    #[validate(length(min = 3))]      
    name: String,
    #[validate(email)]
    email: String
}

fn send_user(user: web::Json<User>) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    client.post("https://jsonplaceholder.typicode.com/users")
            .json(&user.into_inner())
            .send()
}

// Get users
fn get_user() -> impl Responder {
    let mut buf = String::new();
    reqwest::get("https://jsonplaceholder.typicode.com/users")
        .expect("Error in response")
        .read_to_string(&mut buf)
        .expect("Failed to read response");
    let obj: Vec<User> = serde_json::from_str(&mut buf).unwrap();   
    format!("{}\n", serde_json::to_string(&obj).unwrap())
}

// Post user
fn post_user(user: web::Json<User>) -> HttpResponse {
    match user.validate() {
      Ok(_) => {
          match send_user(user) {
            Ok(_)  => return HttpResponse::Ok().body("Successful upload\n"),
            Err(_) => return HttpResponse::UnprocessableEntity().body("!!! Upload error !!!\n")
          }          
      },
      Err(_) => return HttpResponse::UnprocessableEntity().finish()  
    };
}

/* const MAX_SIZE: usize = 1024;
fn post_user2(payload: web::Payload) -> HttpResponse {
    // payload is a stream of Bytes objects
    payload
        .from_err()
        .fold(BytesMut::new(), move |mut body, chunk| {
            if (body.len() + chunk.len()) > MAX_SIZE {
                // Err(error::ErrorBadRequest("overflow"))
            } else {
                body.extend_from_slice(&chunk);
                // Ok(body)
            }
        })
        .and_then(|body| {
            let obj = serde_json::from_slice::<User>(&body)?;
            Ok(HttpResponse::Ok().json(obj))
        }) 
}  */   

fn main() {
    println!("Start server");
    HttpServer::new(|| {
        App::new()
            .route("/api/v1/users", web::get().to(get_user))
            .route("/api/v1/users", web::post().to(post_user))
            //.route("/api/v1/users2", web::post().to(post_user2))
    })
    .bind("0.0.0.0:3020")
    .expect("Can not bind to port 3020")
    .run()
    .unwrap();
}