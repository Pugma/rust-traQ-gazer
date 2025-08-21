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
pub enum MeWordsGetResponse {
    /// Successful retrieval
    Status200_SuccessfulRetrieval
    (models::MyWords)
    ,
    /// Not found
    Status404_NotFound
}


/// MeWords
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait MeWords<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// 自分の登録した単語の一覧.
    ///
    /// MeWordsGet - GET /api/me/words
    async fn me_words_get(
    &self,
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      header_params: &models::MeWordsGetHeaderParams,
    ) -> Result<MeWordsGetResponse, E>;
}
