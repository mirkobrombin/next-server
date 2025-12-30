use bottles_core::proto::bottles::management_server::ManagementServer;
use bottles_server::{BottlesService, state::AppState};
use std::sync::{Arc, RwLock};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info"))
        )
        .init();

    // Initialize State
    let data_path = std::path::PathBuf::from("/home/mirko/.local/share/bottles/next");
    let state = Arc::new(RwLock::new(AppState::new(data_path)));

    let addr = "[::1]:50052".parse().unwrap();
    let service = BottlesService::new(state);
    
    tracing::info!("Bottles Next Server listening on {}", addr);

    tonic::transport::Server::builder()
        .add_service(ManagementServer::new(service))
        // .add_service(ConfigurationServer::new(...)) // To be implemented
        .serve(addr)
        .await?;

    Ok(())
}
