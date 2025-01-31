use tonic::{Request, Response, Status, transport::Server};
use crate::{proto_ext_authz::{authorization_server::{Authorization, AuthorizationServer as ProtoAuthorizationServer}, CheckRequest, CheckResponse}, response::invalid, redis_validate::validate_resource_status};

pub mod response;
pub mod redis_validate;
pub mod lts_validate;
pub mod proto_media_lts{tonic::include_proto!("medialts");}
pub mod proto_ext_authz{tonic::include_proto!("extauthz");}


#[tokio::main]
async fn main() {
    redis_validate::_setup().await.unwrap();
    lts_validate::_setup().await.unwrap();
    let addr = "[::1]:10000".parse().unwrap();
    let svc = ProtoAuthorizationServer::new(AuthorizationServer);
    match Server::builder().add_service(svc).serve(addr).await{
    Ok(_) => todo!(),
    Err(_) => todo!(),
    };
}


struct AuthorizationServer;
#[tonic::async_trait]
impl Authorization for AuthorizationServer{
   async fn check(&self, request: Request<CheckRequest>) -> Result<Response<CheckResponse>, Status>{
    let check_req = &request.into_inner();
    match check_req.method.eq("GET"){
        true => validate_resource_status(&check_req.path).await,
        false => invalid(),
    }
   }
}
