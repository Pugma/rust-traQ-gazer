use super::Handler;
use axum::http::Method;
use axum_extra::extract::CookieJar;
use axum_extra::extract::Host;
use openapi::{
    apis::words::{
        Words, WordsGetResponse, WordsMeGetResponse, WordsPostResponse, WordsWordIdDeleteResponse,
        WordsWordIdPutResponse,
    },
    models,
};

#[async_trait::async_trait]
impl Words for Handler {
    async fn words_get(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        query_params: &models::WordsGetQueryParams,
    ) -> Result<WordsGetResponse, ()> {
        // クエリパラメータの有無で動作を変更
        let word = if let Some(word) = (&query_params).word.clone() {
            self.repo.get_by_word(word).await
        } else if let Some(trap_id) = (&query_params).trap_id.clone() {
            self.repo.get_by_user(trap_id).await
        } else {
            self.repo.get_all().await
        };

        match word {
            Ok(words) => Ok(WordsGetResponse::Status200_SuccessfulRetrieval(words)),
            Err(_) => Err(()),
        }
    }

    async fn words_me_get(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        header_params: &models::WordsMeGetHeaderParams,
    ) -> Result<WordsMeGetResponse, ()> {
        let result = self
            .repo
            .get_my_word(header_params.x_forwarded_user.clone())
            .await;

        match result {
            Ok(i) => Ok(WordsMeGetResponse::Status200_SuccessfulRetrieval(i)),
            Err(_) => Ok(WordsMeGetResponse::Status400_InvalidInput),
        }
    }

    async fn words_post(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        header_params: &models::WordsPostHeaderParams,
        body: &models::NewWord,
    ) -> Result<WordsPostResponse, ()> {
        let result = self
            .repo
            .register(header_params.x_forwarded_user.clone(), body.word.clone())
            .await;

        match result {
            Ok(()) => Ok(WordsPostResponse::Status200_SuccessfulRegistration),
            Err(_) => Ok(WordsPostResponse::Status400_InvalidInput),
        }
    }

    async fn words_word_id_delete(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        header_params: &models::WordsWordIdDeleteHeaderParams,
        path_params: &models::WordsWordIdDeletePathParams,
    ) -> Result<WordsWordIdDeleteResponse, ()> {
        let result = self
            .repo
            .delete(header_params.x_forwarded_user.clone(), path_params.word_id)
            .await;

        match result {
            Ok(()) => Ok(WordsWordIdDeleteResponse::Status200_SuccessfulDeletion),
            Err(_) => Ok(WordsWordIdDeleteResponse::Status404_NotFound),
        }
    }

    async fn words_word_id_put(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        _header_params: &models::WordsWordIdPutHeaderParams,
        path_params: &models::WordsWordIdPutPathParams,
        body: &models::ExcludedUsers,
    ) -> Result<WordsWordIdPutResponse, ()> {
        let result = self
            .repo
            .edit_excluded_users(path_params.word_id, body.clone())
            .await;

        match result {
            Ok(()) => Ok(WordsWordIdPutResponse::Status200_SuccessfulEdit),
            Err(_) => Ok(WordsWordIdPutResponse::Status404_NotFound),
        }
    }
}
