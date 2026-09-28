use tracing::info;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    info!("Cache server (placeholder)");
    info!("This will be implemented later with Axum/Actix-web");
    info!("For now, use cache-cli to interact with the database");
}
