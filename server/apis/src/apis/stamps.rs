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
pub enum StampsGetResponse {
    /// Successful retrieval
    Status200_SuccessfulRetrieval
    (models::Stamps)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum StampsPostResponse {
    /// Successful registration
    Status201_SuccessfulRegistration
    ,
    /// Invalid input
    Status400_InvalidInput
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum StampsStampIdDeleteResponse {
    /// Successful deletion
    Status204_SuccessfulDeletion
    ,
    /// Not found
    Status404_NotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum StampsStampIdExclusionsPutResponse {
    /// Successful edit
    Status200_SuccessfulEdit
    ,
    /// Not found
    Status404_NotFound
}


/// Stamps
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Stamps<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// スタンプの一覧.
    ///
    /// StampsGet - GET /api/stamps
    async fn stamps_get(
    &self,
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      header_params: &models::StampsGetHeaderParams,
    ) -> Result<StampsGetResponse, E>;

    /// スタンプの登録.
    ///
    /// StampsPost - POST /api/stamps
    async fn stamps_post(
    &self,
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      header_params: &models::StampsPostHeaderParams,
            body: &models::NewStamp,
    ) -> Result<StampsPostResponse, E>;

    /// スタンプの削除.
    ///
    /// StampsStampIdDelete - DELETE /api/stamps/{stampId}
    async fn stamps_stamp_id_delete(
    &self,
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      header_params: &models::StampsStampIdDeleteHeaderParams,
      path_params: &models::StampsStampIdDeletePathParams,
    ) -> Result<StampsStampIdDeleteResponse, E>;

    /// スタンプの通知除外者の設定.
    ///
    /// StampsStampIdExclusionsPut - PUT /api/stamps/{stampId}/exclusions
    async fn stamps_stamp_id_exclusions_put(
    &self,
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      header_params: &models::StampsStampIdExclusionsPutHeaderParams,
      path_params: &models::StampsStampIdExclusionsPutPathParams,
            body: &models::ExcludedUsers,
    ) -> Result<StampsStampIdExclusionsPutResponse, E>;
}
