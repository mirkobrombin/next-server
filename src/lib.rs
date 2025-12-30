use bottles_core::proto::bottles::{self, management_server::{self, Management}};
use bottles_core::bottle::{Bottle, BottleType};
use crate::state::SharedState;
use tonic::{Request, Response, Status};

pub mod state;

pub struct BottlesService {
    state: SharedState,
}

impl BottlesService {
    pub fn new(state: SharedState) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl Management for BottlesService {
    async fn create_bottle(
        &self,
        request: Request<bottles::CreateBottleRequest>,
    ) -> Result<Response<bottles::Bottle>, Status> {
        let req = request.into_inner();
        tracing::info!("Received CreateBottle request for: {}", req.name);
        let name = req.name;
        
        let mut state = self.state.write().map_err(|_| Status::internal("Lock error"))?;
        
        // Validation: Check if exists
        if state.bottles.iter().any(|b| b.name == name) {
            tracing::warn!("Bottle {} already exists", name);
            return Err(Status::already_exists("Bottle already exists"));
        }

        // Logic (stub path for now, usually should be derived from config)
        let path = std::path::PathBuf::from(format!("/home/mirko/.local/share/bottles/bottles/{}", name)); // TODO: use proper config
        let kind = match req.r#type.as_str() {
            "Gaming" => BottleType::Gaming,
            "Software" => BottleType::Software,
            _ => BottleType::Custom,
        };

        let bottle = Bottle::new(name.clone(), path, kind);
        
        // Save
        state.bottles.push(bottle.clone());
        state.save();
        tracing::info!("Bottle {} created successfully", name);

        // Map to Proto
        Ok(Response::new(bottles::Bottle {
            name: bottle.name,
            path: bottle.path.to_string_lossy().to_string(),
            r#type: req.r#type, // Simplified mapping
            active: false,
            config: None, // Default config
        }))
    }

    async fn delete_bottle(
        &self,
        request: Request<bottles::DeleteBottleRequest>,
    ) -> Result<Response<bottles::ResultResponse>, Status> {
        let name = request.into_inner().name;
        tracing::info!("Received DeleteBottle request for: {}", name);
        let mut state = self.state.write().map_err(|_| Status::internal("Lock error"))?;

        if let Some(pos) = state.bottles.iter().position(|b| b.name == name) {
            state.bottles.remove(pos);
            state.save();
            tracing::info!("Bottle {} deleted successfully", name);
            Ok(Response::new(bottles::ResultResponse {
                success: true,
                error_message: String::new(),
            }))
        } else {
            tracing::warn!("Bottle {} not found for deletion", name);
            Err(Status::not_found("Bottle not found"))
        }
    }

    async fn list_bottles(
        &self,
        _request: Request<bottles::ListBottlesRequest>,
    ) -> Result<Response<bottles::ListBottlesResponse>, Status> {
        tracing::info!("Received ListBottles request");
        let state = self.state.read().map_err(|_| Status::internal("Lock error"))?;
        
        let bottles: Vec<bottles::Bottle> = state.bottles.iter().map(|b| bottles::Bottle {
            name: b.name.clone(),
            path: b.path.to_string_lossy().to_string(),
            r#type: format!("{:?}", b.kind),
            active: b.active,
            config: None,
        }).collect();
        
        tracing::info!("Returning {} bottles", bottles.len());

        Ok(Response::new(bottles::ListBottlesResponse { bottles }))
    }

    async fn get_bottle(
        &self,
        request: Request<bottles::GetBottleRequest>,
    ) -> Result<Response<bottles::Bottle>, Status> {
        let name = request.into_inner().name;
        let state = self.state.read().map_err(|_| Status::internal("Lock error"))?;

        let bottle = state.bottles.iter().find(|b| b.name == name)
            .ok_or_else(|| Status::not_found("Bottle not found"))?;

        Ok(Response::new(bottles::Bottle {
            name: bottle.name.clone(),
            path: bottle.path.to_string_lossy().to_string(),
            r#type: format!("{:?}", bottle.kind),
            active: bottle.active,
            config: None,
        }))
    }

    async fn start_bottle(
        &self,
        _request: Request<bottles::BottleRequest>,
    ) -> Result<Response<bottles::ResultResponse>, Status> {
        // TODO: Implement Agent Launching Logic
        Err(Status::unimplemented("Not implemented yet"))
    }

    async fn stop_bottle(
        &self,
        _request: Request<bottles::BottleRequest>,
    ) -> Result<Response<bottles::ResultResponse>, Status> {
        Err(Status::unimplemented("Not implemented yet"))
    }

    async fn restart_bottle(
        &self,
        _request: Request<bottles::BottleRequest>,
    ) -> Result<Response<bottles::ResultResponse>, Status> {
        Err(Status::unimplemented("Not implemented yet"))
    }
}
