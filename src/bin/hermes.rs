use hermes_kv::{app::server_builder, config::GlobalConfig, logger};
use prometheus::{Counter, Opts};

#[tokio::main]
async fn main() {
    let registry = prometheus::Registry::new();

    // Define a counter for the health check hits
    let counter_opts = Opts::new("health_check_hits", "Number of health check requests");
    let health_check_counter = Counter::with_opts(counter_opts).unwrap();
    registry
        .register(Box::new(health_check_counter.clone()))
        .unwrap();

    // Set up Prometheus exporter with the registry
    let _exporter = opentelemetry_prometheus::exporter()
        .with_registry(registry.clone())
        .build()
        .expect("unable to start monitoring");

    let configs = GlobalConfig::default();
    let _guard = logger::setup(&configs.log);
    logger::warn!("Server started with {configs:?}");
    let server = server_builder(configs, health_check_counter)
        .await
        .expect("Failed to create the server");
    let _ = server.await;
}
