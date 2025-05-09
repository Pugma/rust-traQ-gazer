use axum::http::Method;
use axum_extra::extract::{CookieJar, Host};
use openapi::apis::trend::{
    GetTodayTrendingWordsResponse, GetTrendingWordsForDayResponse,
    GetTrendingWordsForMonthResponse, GetTrendingWordsForYearResponse, Trend,
};

use super::Handler;

#[async_trait::async_trait]
impl Trend for Handler {
    async fn get_today_trending_words(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        _query_params: &openapi::models::GetTodayTrendingWordsQueryParams,
    ) -> Result<GetTodayTrendingWordsResponse, ()> {
        unimplemented!()
    }

    async fn get_trending_words_for_day(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        _path_params: &openapi::models::GetTrendingWordsForDayPathParams,
        _query_params: &openapi::models::GetTrendingWordsForDayQueryParams,
    ) -> Result<GetTrendingWordsForDayResponse, ()> {
        unimplemented!()
    }

    async fn get_trending_words_for_month(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        _path_params: &openapi::models::GetTrendingWordsForMonthPathParams,
        _query_params: &openapi::models::GetTrendingWordsForMonthQueryParams,
    ) -> Result<GetTrendingWordsForMonthResponse, ()> {
        unimplemented!()
    }

    async fn get_trending_words_for_year(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        _path_params: &openapi::models::GetTrendingWordsForYearPathParams,
        _query_params: &openapi::models::GetTrendingWordsForYearQueryParams,
    ) -> Result<GetTrendingWordsForYearResponse, ()> {
        unimplemented!()
    }
}
