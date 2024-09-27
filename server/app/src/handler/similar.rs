use axum::async_trait;
use openapi::apis::similar::{
    GetRecommendedWordsForUserResponse, GetUsersWithSimilarWordsResponse, Similar,
};

use super::Handler;

#[async_trait]
impl Similar for Handler {
    async fn get_recommended_words_for_user(
        &self,
        _method: axum::http::Method,
        _host: axum::extract::Host,
        _cookies: axum_extra::extract::CookieJar,
        _path_params: openapi::models::GetRecommendedWordsForUserPathParams,
    ) -> Result<GetRecommendedWordsForUserResponse, String> {
        unimplemented!()
    }

    async fn get_users_with_similar_words(
        &self,
        _method: axum::http::Method,
        _host: axum::extract::Host,
        _cookies: axum_extra::extract::CookieJar,
        _path_params: openapi::models::GetUsersWithSimilarWordsPathParams,
    ) -> Result<GetUsersWithSimilarWordsResponse, String> {
        unimplemented!()
    }
}
