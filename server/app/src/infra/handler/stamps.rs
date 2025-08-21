use axum::http::Method;
use axum_extra::extract::{CookieJar, Host};
use openapi::{
    apis::stamps::{
        Stamps, StampsGetResponse, StampsPostResponse, StampsStampIdDeleteResponse,
        StampsStampIdExclusionsPutResponse,
    },
    models::{
        ExcludedUsers, NewStamp, StampsGetHeaderParams, StampsPostHeaderParams,
        StampsStampIdDeleteHeaderParams, StampsStampIdDeletePathParams,
        StampsStampIdExclusionsPutHeaderParams, StampsStampIdExclusionsPutPathParams,
    },
};

use super::Handler;

#[async_trait::async_trait]
impl Stamps for Handler {
    async fn stamps_get(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        _header_params: &StampsGetHeaderParams,
    ) -> Result<StampsGetResponse, ()> {
        unimplemented!()
    }

    async fn stamps_post(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        _header_params: &StampsPostHeaderParams,
        _body: &NewStamp,
    ) -> Result<StampsPostResponse, ()> {
        unimplemented!()
    }

    async fn stamps_stamp_id_delete(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        _header_params: &StampsStampIdDeleteHeaderParams,
        _path_params: &StampsStampIdDeletePathParams,
    ) -> Result<StampsStampIdDeleteResponse, ()> {
        unimplemented!()
    }

    async fn stamps_stamp_id_exclusions_put(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        _header_params: &StampsStampIdExclusionsPutHeaderParams,
        _path_params: &StampsStampIdExclusionsPutPathParams,
        _body: &ExcludedUsers,
    ) -> Result<StampsStampIdExclusionsPutResponse, ()> {
        unimplemented!()
    }
}
