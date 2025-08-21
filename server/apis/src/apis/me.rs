use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::{CookieJar, Host};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum MeGetResponse {
    /// Successful retrieval
    Status200_SuccessfulRetrieval
    (models::MyInfo)
    ,
    /// Not found
    Status404_NotFound
}


/// Me
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Me<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// 自分の情報の閲覧.
    ///
    /// MeGet - GET /api/me
    async fn me_get(
    &self,
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      header_params: &models::MeGetHeaderParams,
    ) -> Result<MeGetResponse, E>;
}
