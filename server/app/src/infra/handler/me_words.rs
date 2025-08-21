use axum::http::Method;
use axum_extra::extract::{CookieJar, Host};
use openapi::{
    apis::me_words::{MeWords, MeWordsGetResponse},
    models::MeWordsGetHeaderParams,
};

use crate::infra::handler::Handler;

#[async_trait::async_trait]
impl MeWords for Handler {
    async fn me_words_get(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        _header_params: &MeWordsGetHeaderParams,
    ) -> Result<MeWordsGetResponse, ()> {
        unimplemented!()
    }
}
