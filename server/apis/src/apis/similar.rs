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
pub enum GetRecommendedWordsForUserResponse {
    /// OK
    Status200_OK
    (models::RecommendedWords)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetUsersWithSimilarWordsResponse {
    /// OK
    Status200_OK
    (models::SimilarUsers)
}


/// Similar
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Similar<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// おすすめの単語を出す.
    ///
    /// GetRecommendedWordsForUser - GET /api/similar/{userId}/recommend
    async fn get_recommended_words_for_user(
    &self,
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      path_params: &models::GetRecommendedWordsForUserPathParams,
    ) -> Result<GetRecommendedWordsForUserResponse, E>;

    /// 似たような者を探す.
    ///
    /// GetUsersWithSimilarWords - GET /api/similar/{userId}
    async fn get_users_with_similar_words(
    &self,
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      path_params: &models::GetUsersWithSimilarWordsPathParams,
    ) -> Result<GetUsersWithSimilarWordsResponse, E>;
}
