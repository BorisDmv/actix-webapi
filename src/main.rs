use actix_web::{get, web, App, HttpServer, HttpResponse, HttpRequest, Responder, Error};
use actix::{Actor, StreamHandler};
use actix_web_actors::ws;


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



struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    println!("{:?}", resp);
    resp
}


#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(greet)
            .service(get_data)
            .route("/ws/", web::get().to(index))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}