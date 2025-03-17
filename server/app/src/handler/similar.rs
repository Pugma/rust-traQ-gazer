use axum::http::Method;
use axum_extra::extract::{CookieJar, Host};
use openapi::apis::similar::{
    GetRecommendedWordsForUserResponse, GetUsersWithSimilarWordsResponse, Similar,
};
use openapi::models;

use super::Handler;

#[async_trait::async_trait]
impl Similar for Handler {
    async fn get_recommended_words_for_user(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        _path_params: &models::GetRecommendedWordsForUserPathParams,
    ) -> Result<GetRecommendedWordsForUserResponse, ()> {
        unimplemented!()
    }

    async fn get_users_with_similar_words(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        _path_params: &models::GetUsersWithSimilarWordsPathParams,
    ) -> Result<GetUsersWithSimilarWordsResponse, ()> {
        unimplemented!()
    }
}
