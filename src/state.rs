use bottles_core::bottle::Bottle;
use bottles_core::persistence::Persistence;
use std::sync::{Arc, RwLock};
use std::path::PathBuf;

pub struct AppState {
    pub bottles: Vec<Bottle>,
    pub persistence: Persistence,
}

impl AppState {
    pub fn new(data_path: PathBuf) -> Self {
        let persistence = Persistence::new(data_path);
        let bottles = persistence.load_bottles().unwrap_or_else(|e| {
            tracing::error!("Failed to load bottles: {}", e);
            Vec::new()
        });

        Self {
            bottles,
            persistence,
        }
    }

    pub fn save(&self) {
        if let Err(e) = self.persistence.save_bottles(&self.bottles) {
            tracing::error!("Failed to save bottles: {}", e);
        }
    }
}

pub type SharedState = Arc<RwLock<AppState>>;
