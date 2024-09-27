use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::{CookieJar, Multipart};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetTodayTrendingWordsResponse {
    /// OK
    Status200_OK(models::TrendingWords),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetTrendingWordsForDayResponse {
    /// OK
    Status200_OK(models::TrendingWords),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetTrendingWordsForMonthResponse {
    /// OK
    Status200_OK(models::TrendingWords),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetTrendingWordsForYearResponse {
    /// OK
    Status200_OK(models::TrendingWords),
}

/// Trend
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Trend {
    /// 今日のトレンド.
    ///
    /// GetTodayTrendingWords - GET /api/trend/day/today
    async fn get_today_trending_words(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        query_params: models::GetTodayTrendingWordsQueryParams,
    ) -> Result<GetTodayTrendingWordsResponse, String>;

    /// ある日のトレンド.
    ///
    /// GetTrendingWordsForDay - GET /api/trend/day/{day}
    async fn get_trending_words_for_day(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::GetTrendingWordsForDayPathParams,
        query_params: models::GetTrendingWordsForDayQueryParams,
    ) -> Result<GetTrendingWordsForDayResponse, String>;

    /// ある月のトレンド.
    ///
    /// GetTrendingWordsForMonth - GET /api/trend/month/{month}
    async fn get_trending_words_for_month(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::GetTrendingWordsForMonthPathParams,
        query_params: models::GetTrendingWordsForMonthQueryParams,
    ) -> Result<GetTrendingWordsForMonthResponse, String>;

    /// ある年のトレンド.
    ///
    /// GetTrendingWordsForYear - GET /api/trend/year/{year}
    async fn get_trending_words_for_year(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::GetTrendingWordsForYearPathParams,
        query_params: models::GetTrendingWordsForYearQueryParams,
    ) -> Result<GetTrendingWordsForYearResponse, String>;
}
