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
        unimplemented!()
    }

    async fn words_me_get(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        header_params: &models::WordsMeGetHeaderParams,
    ) -> Result<WordsMeGetResponse, ()> {
        unimplemented!()
    }

    async fn words_post(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        header_params: &models::WordsPostHeaderParams,
        body: &models::NewWord,
    ) -> Result<WordsPostResponse, ()> {
        unimplemented!()
    }

    async fn words_word_id_delete(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        header_params: &models::WordsWordIdDeleteHeaderParams,
        path_params: &models::WordsWordIdDeletePathParams,
    ) -> Result<WordsWordIdDeleteResponse, ()> {
        unimplemented!()
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
        unimplemented!()
    }
}
