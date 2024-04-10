use std::env;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    HttpResponse::Ok().body(format!("Hello {app_name}!"))
}

// This struct represents state
#[derive(Default)]
struct AppState {
    app_name: String,
    azure_scope: String,
}

impl AppState {
    pub fn new() -> Self {
        let azure_scope = env::var("AZURE_SCOPE").unwrap_or_default();
        Self {
            app_name: "balls".to_string(),
            azure_scope
        }
    }
}

// impl Default for AppState {
//     fn default() -> Self {
//         Self {
//             app_name: "".to_string(),
//             azure_scope: "".to_string(),
//         }
//     }
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState::new()))
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}