use std::collections::HashSet;
use std::sync::{Arc, Mutex};

/// The Orchestrator manages the lifecycle of bottles.
///
/// TODO: This implementation currently serves as a functional mock to validate the gRPC
/// protocol and orchestration flow. It does not yet integrate with the actual Component Manager
/// or spawn real processes (Agent:WineBridge).
///
/// Future implementation requirements:
/// - Integration with Component Manager for runner/dependency resolution.
/// - Spawning of actual Agent processes.
#[derive(Clone)]
pub struct Orchestrator {
    // In a real implementation, this would hold handles to Child processes or gRPC clients.
    running_bottles: Arc<Mutex<HashSet<String>>>,
}

impl Orchestrator {
    pub fn new() -> Self {
        Self {
            running_bottles: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    pub fn start_bottle(&self, name: String) -> Result<(), String> {
        let mut bottles = self.running_bottles.lock().map_err(|_| "Lock poisoning")?;

        if bottles.contains(&name) {
            return Err("Bottle is already running".to_string());
        }

        // TODO: Real process spawning logic goes here.
        // - Resolve Runner via Component Manager.
        // - Launch Agent process.
        // - Wait for Agent readiness.
        
        bottles.insert(name);
        Ok(())
    }

    pub fn stop_bottle(&self, name: &str) -> Result<(), String> {
        let mut bottles = self.running_bottles.lock().map_err(|_| "Lock poisoning")?;

        if !bottles.remove(name) {
             return Err("Bottle is not running".to_string());
        }
        
        Ok(())
    }

    pub fn is_running(&self, name: &str) -> bool {
        let bottles = self.running_bottles.lock().unwrap();
        bottles.contains(name)
    }
}
