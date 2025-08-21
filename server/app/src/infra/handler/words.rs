use super::Handler;
use axum::http::Method;
use axum_extra::extract::CookieJar;
use axum_extra::extract::Host;
use openapi::{
    apis::words::{
        Words, WordsPostResponse, WordsWordIdDeleteResponse, WordsWordIdExclusionsPutResponse,
    },
    models::{
        ExcludedUsers, NewWord, WordsPostHeaderParams, WordsWordIdDeleteHeaderParams,
        WordsWordIdDeletePathParams, WordsWordIdExclusionsPutHeaderParams,
        WordsWordIdExclusionsPutPathParams,
    },
};

#[async_trait::async_trait]
impl Words for Handler {
    async fn words_post(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        _header_params: &WordsPostHeaderParams,
        _body: &NewWord,
    ) -> Result<WordsPostResponse, ()> {
        unimplemented!()
    }

    async fn words_word_id_delete(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        _header_params: &WordsWordIdDeleteHeaderParams,
        _path_params: &WordsWordIdDeletePathParams,
    ) -> Result<WordsWordIdDeleteResponse, ()> {
        unimplemented!()
    }

    async fn words_word_id_exclusions_put(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        _header_params: &WordsWordIdExclusionsPutHeaderParams,
        _path_params: &WordsWordIdExclusionsPutPathParams,
        _body: &ExcludedUsers,
    ) -> Result<WordsWordIdExclusionsPutResponse, ()> {
        unimplemented!()
    }
}
