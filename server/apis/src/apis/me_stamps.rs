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
pub enum MeStampsGetResponse {
    /// Successful retrieval
    Status200_SuccessfulRetrieval
    (models::MyStamps)
    ,
    /// Not found
    Status404_NotFound
}


/// MeStamps
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait MeStamps<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// 自分の登録したスタンプの一覧.
    ///
    /// MeStampsGet - GET /api/me/stamps
    async fn me_stamps_get(
    &self,
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      header_params: &models::MeStampsGetHeaderParams,
    ) -> Result<MeStampsGetResponse, E>;
}
