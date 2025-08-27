use actix_web::{App, HttpServer};
use opentelemetry_instrumentation_actix_web::RequestTracing;

mod db;
mod handlers;
mod telemetry_conf;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    telemetry_conf::init_otel().expect("Failed to initialize OpenTelemetry");

    HttpServer::new(move || {
        App::new()
            .wrap(RequestTracing::new())
            .service(handlers::songs)
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}
