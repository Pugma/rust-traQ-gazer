use axum::async_trait;
use openapi::{
    apis::stamps::{Stamps, StampsGetResponse, StampsPostResponse},
    models,
};

use super::Handler;

#[async_trait]
impl Stamps for Handler {
    async fn stamps_get(
        &self,
        _method: axum::http::Method,
        _host: axum::extract::Host,
        _cookies: axum_extra::extract::CookieJar,
        _query_params: openapi::models::StampsGetQueryParams,
    ) -> Result<StampsGetResponse, String> {
        unimplemented!()
    }

    async fn stamps_post(
        &self,
        _method: axum::http::Method,
        _host: axum::extract::Host,
        _cookies: axum_extra::extract::CookieJar,
        _header_params: models::StampsPostHeaderParams,
        _body: openapi::models::NewStamp,
    ) -> Result<StampsPostResponse, String> {
        unimplemented!()
    }
}
