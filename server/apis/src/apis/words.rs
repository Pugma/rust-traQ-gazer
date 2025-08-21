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
pub enum WordsPostResponse {
    /// Successful registration
    Status201_SuccessfulRegistration
    ,
    /// Invalid input
    Status400_InvalidInput
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum WordsWordIdDeleteResponse {
    /// Successful deletion
    Status204_SuccessfulDeletion
    ,
    /// Not found
    Status404_NotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum WordsWordIdExclusionsPutResponse {
    /// Successful edit
    Status200_SuccessfulEdit
    ,
    /// Not found
    Status404_NotFound
}


/// Words
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Words<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// 単語の登録.
    ///
    /// WordsPost - POST /api/words
    async fn words_post(
    &self,
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      header_params: &models::WordsPostHeaderParams,
            body: &models::NewWord,
    ) -> Result<WordsPostResponse, E>;

    /// 単語の削除.
    ///
    /// WordsWordIdDelete - DELETE /api/words/{wordId}
    async fn words_word_id_delete(
    &self,
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      header_params: &models::WordsWordIdDeleteHeaderParams,
      path_params: &models::WordsWordIdDeletePathParams,
    ) -> Result<WordsWordIdDeleteResponse, E>;

    /// 単語の通知除外者の設定.
    ///
    /// WordsWordIdExclusionsPut - PUT /api/words/{wordId}/exclusions
    async fn words_word_id_exclusions_put(
    &self,
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      header_params: &models::WordsWordIdExclusionsPutHeaderParams,
      path_params: &models::WordsWordIdExclusionsPutPathParams,
            body: &models::ExcludedUsers,
    ) -> Result<WordsWordIdExclusionsPutResponse, E>;
}
