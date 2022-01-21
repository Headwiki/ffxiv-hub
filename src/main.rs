//! main.rs

use std::net::TcpListener;
use ffxiv_hub::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run(
        TcpListener::bind("127.0.0.1:8080").
            expect("Failed to bind random port"))
        ?.await
}