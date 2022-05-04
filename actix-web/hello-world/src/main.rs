use std::{
    cell::Cell,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};
use tokio::time::Duration;

use futures::{future::ok, stream::once};

use actix_web::{
    body::BoxBody, get, http::header::ContentType, post, web, App, Error, HttpRequest,
    HttpResponse, HttpServer, Responder,
};

use serde::Serialize;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = AppState {
        local_count: Cell::new(0),
        global_count: Arc::new(AtomicUsize::new(0)),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(data.clone()))
            .service(show_count)
            .service(add_one)
            .service(user)
            .service(stream)
            .service(echo)
            .configure(config)
            .service(web::scope("/api").configure(scoped_config))
            .service(
                // prefixes all resources and routes attached to it...
                web::scope("/app")
                    // ...so this handles requests for `GET /app/index.html`
                    .route("/index.html", web::get().to(index_html)),
            )
            .route("/hey", web::get().to(manual_hello))
    })
    .workers(4)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[derive(Serialize)]
struct MyObj {
    name: &'static str,
}
// Responder
impl Responder for MyObj {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

#[get("/user")]
async fn user() -> impl Responder {
    MyObj { name: "user" }
}

#[get("/stream")]
async fn stream() -> HttpResponse {
    let body = once(ok::<_, Error>(web::Bytes::from_static(b"test")));

    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn index_html() -> impl Responder {
    // std::thread::sleep(Duration::from_secs(5)); // <-- Bad practice! Will cause the current worker thread to hang!
    tokio::time::sleep(Duration::from_secs(5)).await; // <-- Ok. Worker thread will handle other requests here
    web::Bytes::from_static(b"Hello world!")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

// this function could be located in a different module
fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(|| async { HttpResponse::Ok().body("test") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

// this function could be located in a different module
fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/apps")
            .route(web::get().to(|| async { HttpResponse::Ok().body("apps") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

#[derive(Clone)]
struct AppState {
    local_count: Cell<usize>,
    global_count: Arc<AtomicUsize>,
}

#[get("/")]
async fn show_count(data: web::Data<AppState>) -> impl Responder {
    format!(
        "global_count: {}\nlocal_count: {}",
        data.global_count.load(Ordering::Relaxed),
        data.local_count.get()
    )
}

#[get("/add")]
async fn add_one(data: web::Data<AppState>) -> impl Responder {
    data.global_count.fetch_add(1, Ordering::Relaxed);

    let local_count = data.local_count.get();
    data.local_count.set(local_count + 1);

    format!(
        "global_count: {}\nlocal_count: {}",
        data.global_count.load(Ordering::Relaxed),
        data.local_count.get()
    )
}
