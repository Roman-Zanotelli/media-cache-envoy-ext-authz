use dashmap::DashMap;
use once_cell::sync::OnceCell;
use redis::{AsyncCommands, RedisError};
use tonic::{Response, Status};

use crate::{lts_validate::request_lts, proto_ext_authz::CheckResponse, response::{internal_err, loading, ok, wrap_response}};
static REDIS_INSTANCE: OnceCell<redis::aio::MultiplexedConnection> = OnceCell::new();
static MEM_INSTANCE: OnceCell<DashMap<String, CheckResponse>> = OnceCell::new();
pub async fn _setup() -> Result<(),()>{
    let redis_client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let redis_con = redis_client.get_multiplexed_async_connection().await.unwrap();

    let _ = REDIS_INSTANCE.set(redis_con);
    Ok(())
}

pub async fn validate_resource_status(path: &str) -> Result<Response<CheckResponse>, Status>{
    match validate_redis(path).await{
        Ok(resp) => Ok(resp),
        Err(_) => internal_err(),
    }
}

pub async fn validate_redis(path: &str) -> Result<Response<CheckResponse>, Status> {
    let mut con = REDIS_INSTANCE.get().unwrap().clone();
    match MEM_INSTANCE.get().unwrap().get(path){
        Some(val) => wrap_response(val.value()),
        None => match con.exists::<&str, bool>(path).await{
            Ok(exists) => match exists{
                true => match  con.get::<&str, bool>(path).await{
                    Ok(ready) => match ready {
                        true => ok(),
                        false => loading(),
                    },
                    Err(redis_err) => handle_redis_err(redis_err).await,
                },
                false => request_lts(path).await,
            },
            Err(redis_err) => handle_redis_err(redis_err).await,
        }
    }
}


async fn handle_redis_err(err: RedisError) -> Result<Response<CheckResponse>, Status>{
    //logic for handing an internal error

    internal_err()
}