use axum::http::Method;
use axum_extra::extract::{CookieJar, Host};
use openapi::{
    apis::me_stamps::{MeStamps, MeStampsGetResponse},
    models::MeStampsGetHeaderParams,
};

use crate::infra::handler::Handler;

#[async_trait::async_trait]
impl MeStamps for Handler {
    async fn me_stamps_get(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        _header_params: &MeStampsGetHeaderParams,
    ) -> Result<MeStampsGetResponse, ()> {
        unimplemented!()
    }
}
