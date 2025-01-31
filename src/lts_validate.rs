use once_cell::sync::OnceCell;
use tonic::Response;

use crate::{proto_media_lts::{media_long_term_storage_server_client::MediaLongTermStorageServerClient, LtsResponse}, response::{internal_err, loading, not_found}};


static LTS_INSTANCE: OnceCell<MediaLongTermStorageServerClient<tonic::transport::Channel>> = OnceCell::new();

pub async fn _setup() -> Result<(),()>{
    Ok(LTS_INSTANCE.set(MediaLongTermStorageServerClient::connect("dst").await.unwrap()).unwrap())
}
pub async fn request_lts(resource: &str) -> Result<tonic::Response<crate::proto_ext_authz::CheckResponse>, tonic::Status>{
    let mut con = LTS_INSTANCE.get().unwrap().clone();
        match con.request_resource(crate::proto_media_lts::LtsRequest{resource: resource.to_string()}).await{
            Ok(response) => process_response(response).await,
            Err(con_err) => internal_err(),
        }
}
async fn process_response(response: Response<LtsResponse>) -> Result<tonic::Response<crate::proto_ext_authz::CheckResponse>, tonic::Status>{
    //in here we should check if it exists then add it to a dashmap until the wait duration has passed (this is useful to reduce requests to redis and the backend logic server)
    let response = response.into_inner();
    let wait_until = response.wait_until;
    match response.exists{
        true => {
            //spawn a thread to add and then remove itself with the result for wait_until

            return loading();
        },
        false => {
            //spawn a thread to add and then remove itself with the result for wait_until
            
            return not_found();
        }
    }
}