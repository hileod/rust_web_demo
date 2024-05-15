use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use tower_http::cors::{Any, CorsLayer};
mod types;
use types::Person;
#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
        .route("/", get(root))
        //NEW
        .route("/people", get(get_people))
        .layer(cors);


    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {:?}", &listener);

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

// NEW
async fn get_people() -> impl IntoResponse {
    let people = vec![
        Person {
            name: String::from("Person A"),
            age: 36,
            favourite_food: Some(String::from("Pizza")),
        },
        Person {
            name: String::from("Person B"),
            age: 5,
            favourite_food: Some(String::from("Broccoli")),
        },
        Person {
            name: String::from("Person C"),
            age: 100,
            favourite_food: None,
        },
        Person {
            name: String::from("Person d"),
            age: 18,
            favourite_food: None,
        },
    ];

    (StatusCode::OK, Json(people))
}