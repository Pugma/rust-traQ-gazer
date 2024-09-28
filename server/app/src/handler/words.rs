use super::Handler;
use axum::async_trait;
use axum::extract::Host;
use axum::http::Method;
use axum_extra::extract::CookieJar;
use openapi::{
    apis::words::{
        Words, WordsGetResponse, WordsMeGetResponse, WordsPostResponse,
        WordsUsersUserIdGetResponse, WordsWordIdDeleteResponse, WordsWordIdPutResponse,
    },
    models,
};

#[async_trait]
impl Words for Handler {
    async fn words_get(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        query_params: models::WordsGetQueryParams,
    ) -> Result<WordsGetResponse, String> {
        // クエリパラメータの有無で動作を変更
        let word = if let Some(word) = query_params.word {
            self.repo.get_by_word(word).await
        } else {
            self.repo.get_all().await
        };

        match word {
            Ok(words) => Ok(WordsGetResponse::Status200_SuccessfulRetrieval(words)),
            Err(e) => Err(e.to_string()),
        }
    }

    async fn words_me_get(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        header_params: models::WordsMeGetHeaderParams,
    ) -> Result<WordsMeGetResponse, String> {
        let result = self.repo.get_my_word(header_params.x_forwarded_user).await;

        match result {
            Ok(i) => Ok(WordsMeGetResponse::Status200_SuccessfulRetrieval(i)),
            Err(_) => Ok(WordsMeGetResponse::Status400_InvalidInput),
        }
    }

    async fn words_post(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        header_params: models::WordsPostHeaderParams,
        body: models::NewWord,
    ) -> Result<WordsPostResponse, String> {
        let result = self
            .repo
            .register(header_params.x_forwarded_user, body.word)
            .await;

        match result {
            Ok(()) => Ok(WordsPostResponse::Status200_SuccessfulRegistration),
            Err(_) => Ok(WordsPostResponse::Status400_InvalidInput),
        }
    }

    async fn words_users_user_id_get(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: models::WordsUsersUserIdGetPathParams,
    ) -> Result<WordsUsersUserIdGetResponse, String> {
        let result = self.repo.get_by_user(path_params.user_id).await;

        match result {
            Ok(words) => Ok(WordsUsersUserIdGetResponse::Status200_SuccessfulRetrieval(
                words,
            )),
            Err(_) => Ok(WordsUsersUserIdGetResponse::Status404_NotFound),
        }
    }

    async fn words_word_id_delete(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        header_params: models::WordsWordIdDeleteHeaderParams,
        path_params: models::WordsWordIdDeletePathParams,
    ) -> Result<WordsWordIdDeleteResponse, String> {
        let result = self
            .repo
            .delete(header_params.x_forwarded_user, path_params.word_id)
            .await;

        match result {
            Ok(()) => Ok(WordsWordIdDeleteResponse::Status200_SuccessfulDeletion),
            Err(_) => Ok(WordsWordIdDeleteResponse::Status404_NotFound),
        }
    }

    async fn words_word_id_put(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _header_params: models::WordsWordIdPutHeaderParams,
        path_params: models::WordsWordIdPutPathParams,
        body: models::ExcludedUsers,
    ) -> Result<WordsWordIdPutResponse, String> {
        let result = self
            .repo
            .edit_excluded_users(path_params.word_id, body)
            .await;

        match result {
            Ok(()) => Ok(WordsWordIdPutResponse::Status200_SuccessfulEdit),
            Err(_) => Ok(WordsWordIdPutResponse::Status404_NotFound),
        }
    }
}
