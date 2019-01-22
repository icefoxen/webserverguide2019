use actix_web::{server, App, HttpRequest, Responder};

fn greet(_req: &HttpRequest) -> impl Responder {
    format!("Hello world!")
}

fn main() {
    server::new(|| App::new().resource("/", |r| r.f(greet)))
        .bind("127.0.0.1:8888")
        .expect("Can not bind to port 8888")
        .run();
}
