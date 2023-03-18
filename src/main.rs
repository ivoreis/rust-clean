use actix_clean_architecture::presentation::rest_api::factory::create;
use actix_web::HttpServer;

#[cfg(test)]
mod tests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(move || create())
        .workers(4)
        .bind(("127.0.0.1", 8080))?;
    server.run().await
}
