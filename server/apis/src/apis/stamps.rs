use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::{CookieJar, Multipart};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum StampsGetResponse {
    /// Successful retrieval
    Status200_SuccessfulRetrieval(models::Stamps),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum StampsPostResponse {
    /// Successful registration
    Status200_SuccessfulRegistration,
    /// Invalid input
    Status400_InvalidInput,
}

/// Stamps
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Stamps {
    /// 全ユーザーの登録スタンプの閲覧.
    ///
    /// StampsGet - GET /api/stamps
    async fn stamps_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        query_params: models::StampsGetQueryParams,
    ) -> Result<StampsGetResponse, String>;

    /// スタンプの登録.
    ///
    /// StampsPost - POST /api/stamps
    async fn stamps_post(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        header_params: models::StampsPostHeaderParams,
        body: models::NewStamp,
    ) -> Result<StampsPostResponse, String>;
}
