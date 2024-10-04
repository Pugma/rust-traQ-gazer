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
pub enum WordsGetResponse {
    /// Successful retrieval
    Status200_SuccessfulRetrieval(models::Words),
    /// Not found
    Status404_NotFound,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum WordsMeGetResponse {
    /// Successful retrieval
    Status200_SuccessfulRetrieval(models::MyWords),
    /// Invalid input
    Status400_InvalidInput,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum WordsPostResponse {
    /// Successful registration
    Status200_SuccessfulRegistration,
    /// Invalid input
    Status400_InvalidInput,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum WordsWordIdDeleteResponse {
    /// Successful deletion
    Status200_SuccessfulDeletion,
    /// Not found
    Status404_NotFound,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum WordsWordIdPutResponse {
    /// Successful edit
    Status200_SuccessfulEdit,
    /// Not found
    Status404_NotFound,
}

/// Words
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Words {
    /// 全ユーザーの登録単語の閲覧.
    ///
    /// WordsGet - GET /api/words
    async fn words_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        query_params: models::WordsGetQueryParams,
    ) -> Result<WordsGetResponse, String>;

    /// 自分の登録単語の閲覧.
    ///
    /// WordsMeGet - GET /api/words/me
    async fn words_me_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        header_params: models::WordsMeGetHeaderParams,
    ) -> Result<WordsMeGetResponse, String>;

    /// 単語の登録.
    ///
    /// WordsPost - POST /api/words
    async fn words_post(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        header_params: models::WordsPostHeaderParams,
        body: models::NewWord,
    ) -> Result<WordsPostResponse, String>;

    /// 単語の削除.
    ///
    /// WordsWordIdDelete - DELETE /api/words/{wordId}
    async fn words_word_id_delete(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        header_params: models::WordsWordIdDeleteHeaderParams,
        path_params: models::WordsWordIdDeletePathParams,
    ) -> Result<WordsWordIdDeleteResponse, String>;

    /// 通知除外者の設定.
    ///
    /// WordsWordIdPut - PUT /api/words/{wordId}
    async fn words_word_id_put(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        header_params: models::WordsWordIdPutHeaderParams,
        path_params: models::WordsWordIdPutPathParams,
        body: models::ExcludedUsers,
    ) -> Result<WordsWordIdPutResponse, String>;
}
