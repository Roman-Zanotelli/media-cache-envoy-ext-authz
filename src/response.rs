use tonic::{Response, Status};

use crate::proto_ext_authz::CheckResponse;

pub fn ok() -> Result<Response<CheckResponse>, Status>{
    Ok(Response::new(CheckResponse{ status: Some(crate::proto_ext_authz::Status{
        code: 200,
        message: "OK".to_owned(),
    }), http_response: None }))
}
pub fn loading() -> Result<Response<CheckResponse>, Status>{
    Ok(Response::new(CheckResponse{ status: Some(crate::proto_ext_authz::Status{
        code: 102,
        message: "Resource loading".to_owned(),
    }), http_response: None }))
}
pub fn not_found() -> Result<Response<CheckResponse>, Status>{
    Ok(Response::new(CheckResponse{ status: Some(crate::proto_ext_authz::Status{
        code: 404,
        message: "Not Found".to_owned(),
    }), http_response: None }))
}
pub fn invalid() -> Result<Response<CheckResponse>, Status>{
    Ok(Response::new(CheckResponse{ status: Some(crate::proto_ext_authz::Status{
        code: 405,
        message: "Invalid Request".to_owned(),
    }), http_response: None }))
}
pub fn internal_err() -> Result<Response<CheckResponse>, Status>{
    Ok(Response::new(CheckResponse{ status: Some(crate::proto_ext_authz::Status{
        code: 503,
        message: "Internal Err".to_owned(),
    }), http_response: None }))
}
pub fn wrap_response(resp :&CheckResponse) -> Result<Response<CheckResponse>, Status>{
    Ok(Response::new(resp.clone()))
}