use axum::http::Method;
use axum_extra::extract::{CookieJar, Host};
use openapi::{
    apis::stamps::{Stamps, StampsGetResponse, StampsPostResponse},
    models,
};

use super::Handler;

#[async_trait::async_trait]
impl Stamps for Handler {
    async fn stamps_get(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        _query_params: &models::StampsGetQueryParams,
    ) -> Result<StampsGetResponse, ()> {
        unimplemented!()
    }

    async fn stamps_post(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        _header_params: &models::StampsPostHeaderParams,
        _body: &models::NewStamp,
    ) -> Result<StampsPostResponse, ()> {
        unimplemented!()
    }
}
