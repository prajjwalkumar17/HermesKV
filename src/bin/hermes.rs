use actix_web::middleware::Logger;
use HermesKV::{app::server_builder, config::GlobalConfig};

#[tokio::main]
async fn main() {
    let configs = GlobalConfig::default();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    println!("Server started with {configs:?}");
    let server = server_builder(configs)
        .await
        .expect("Failed to create the server");
    let _ = server.await;
}
