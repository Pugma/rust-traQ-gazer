use axum::async_trait;
use openapi::apis::trend::{
    GetTodayTrendingWordsResponse, GetTrendingWordsForDayResponse,
    GetTrendingWordsForMonthResponse, GetTrendingWordsForYearResponse, Trend,
};

use super::Handler;

#[async_trait]
impl Trend for Handler {
    async fn get_today_trending_words(
        &self,
        _method: axum::http::Method,
        _host: axum::extract::Host,
        _cookies: axum_extra::extract::CookieJar,
        _query_params: openapi::models::GetTodayTrendingWordsQueryParams,
    ) -> Result<GetTodayTrendingWordsResponse, String> {
        unimplemented!()
    }

    async fn get_trending_words_for_day(
        &self,
        _method: axum::http::Method,
        _host: axum::extract::Host,
        _cookies: axum_extra::extract::CookieJar,
        _path_params: openapi::models::GetTrendingWordsForDayPathParams,
        _query_params: openapi::models::GetTrendingWordsForDayQueryParams,
    ) -> Result<GetTrendingWordsForDayResponse, String> {
        unimplemented!()
    }

    async fn get_trending_words_for_month(
        &self,
        _method: axum::http::Method,
        _host: axum::extract::Host,
        _cookies: axum_extra::extract::CookieJar,
        _path_params: openapi::models::GetTrendingWordsForMonthPathParams,
        _query_params: openapi::models::GetTrendingWordsForMonthQueryParams,
    ) -> Result<GetTrendingWordsForMonthResponse, String> {
        unimplemented!()
    }

    async fn get_trending_words_for_year(
        &self,
        _method: axum::http::Method,
        _host: axum::extract::Host,
        _cookies: axum_extra::extract::CookieJar,
        _path_params: openapi::models::GetTrendingWordsForYearPathParams,
        _query_params: openapi::models::GetTrendingWordsForYearQueryParams,
    ) -> Result<GetTrendingWordsForYearResponse, String> {
        unimplemented!()
    }
}
