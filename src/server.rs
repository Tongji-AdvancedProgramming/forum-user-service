use tonic::{Request, Response, Status};
use crate::server::user_service::{LoginRequest, LoginResponse, RegisterRequest, RegisterResponse};
use crate::server::user_service::user_service_server::UserService;

pub mod user_service {
    tonic::include_proto!("user_service");
}

#[derive(Debug, Default)]
pub struct UserServiceProvider {}

#[tonic::async_trait]
impl UserService for UserServiceProvider {
    async fn login(&self, _request: Request<LoginRequest>) -> Result<Response<LoginResponse>, Status> {
        Ok(Response::new(LoginResponse{
            success: false,
            msg: String::from("开发中，敬请期待"),
            user_registered: false,
            user_serialized: "{}".to_string(),
        }))
    }

    async fn register(&self, _request: Request<RegisterRequest>) -> Result<Response<RegisterResponse>, Status> {
        Ok(Response::new(RegisterResponse{
            success:false,
            msg:String::from("开发中，敬请期待")
        }))
    }
}
