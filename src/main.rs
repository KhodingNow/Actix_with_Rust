use actix_web::{post, get, web, App, HttpResponse, HttpServer, Responder};

// This struct represents state
struct AppState {
    app_name: String,
} 

// #[get("/")]
// async fn index(data: web::Data<AppState>) -> String {
//     let app_name = &data.app_name; // <- get app_name
//     format!("Hello {app_name}!") // <- response with app_name 
// }

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there")
}

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .app_data(web::Data::new(AppState {
//                 app_name: String::from("Actix Web"),
//             }))
//             .service(index)       

//     })
//     .bind(("0.0.0.0", 8081))?
//     .run()
//     .await
// }

// Shared Mutable State
// use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is neccessary to mutate safely accross threads
}

async fn index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MuteGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {counter}") // <- response with count
}

// register the date in an App:

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     // Note: web::Data created _outside_ HttpServer::new closure
//     let counter = web::Data::new(AppStateWithCounter {
//         counter: Mutex::new(0),
//     });

//     HttpServer::new(move || {
//         // move counter into the closure
//         App::new()
//             .app_data(counter.clone()) // <- register the created data
//             . route("/", web::get().to(index))

//     })
//     .bind(("0.0.0.0", 8081))?
//     .run()
//     .await
// }

// SERVER - The HHTP Server

// use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(HttpResponse::Ok)))
    .bind(("0.0.0.0", 8081))?
    .run()
    .await 
}

// Multi-THREADING -  the server starts a number of HTTP workers

// use actix_web::{web, App, HttpResponse, HttpServer};

// #[actix_web::main]
//async fn main() {

    HttpServer::new(|| App::new().route("/", web::get().to(HttpResponse::Ok))).workers(4);

    // starts 4 workers

    fn my_handler() -> impl Responder {
        std::thread::sleep(Duration::from_secs(5)); // Bad practice! Will cause the current worker thread to hang
        "response"
    }
//}

// for this reason, long, non-CPU operations (e.g I/O, databases ops) sld be expressed as futures or asynchronous function

async fn my_handler() -> impl Responder {
    tokio::time::sleep(Duration::from_secs(5)).await // <- worker thread will accepts  
    "response"
}

// TLS / HTTPS - Actix Web supports two TLS implementations out-the -box: rustls and openssl.
// The rustls crate feature is rustls integration and openssl is for openssl intergration
// include on .TOML

// [dependencies]
// actix-web = { version = "4", features = ["openssl"] }
// openssl = { version = "0.10" }

// use actix_web::{get, App, HttpRequest, HttpServer, Responder};
use openssl::ssl::{SsAcceptor, SslFiletype, SslMethod};

#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    "Welcome "
}