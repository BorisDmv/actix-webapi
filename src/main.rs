use actix_web::{get, web, App, HttpServer, HttpResponse, Responder};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[get("/data")]
async fn get_data() -> impl Responder {
    // Create a simple JSON object
    let data = serde_json::json!({
        "key1": "value1",
        "key2": "value2",
        "key3": 42,
    });

    // Return the JSON data as the response
    HttpResponse::Ok().json(data)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().
            service(greet)
            .service(get_data)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}