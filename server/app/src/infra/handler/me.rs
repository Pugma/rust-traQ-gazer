use axum::http::Method;
use axum_extra::extract::{CookieJar, Host};
use openapi::{
    apis::me::{Me, MeGetResponse},
    models::MeGetHeaderParams,
};

use crate::infra::handler::Handler;

#[async_trait::async_trait]
impl Me for Handler {
    async fn me_get(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        _header_params: &MeGetHeaderParams,
    ) -> Result<MeGetResponse, ()> {
        unimplemented!()
    }
}
