use std::collections::HashMap;

use axum::{body::Body, extract::*, response::Response, routing::*};
use axum_extra::extract::{CookieJar, Host};
use bytes::Bytes;
use http::{header::CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue, Method, StatusCode};
use tracing::error;
use validator::{Validate, ValidationErrors};

use crate::{header, types::*};

#[allow(unused_imports)]
use crate::{apis, models};


/// Setup API Server.
pub fn new<I, A, E>(api_impl: I) -> Router
where
    I: AsRef<A> + Clone + Send + Sync + 'static,
    A: apis::similar::Similar<E> + apis::stamps::Stamps<E> + apis::trend::Trend<E> + apis::words::Words<E> + Send + Sync + 'static,
    E: std::fmt::Debug + Send + Sync + 'static,
    
{
    // build our application with a route
    Router::new()
        .route("/api/similar/{user_id}",
            get(get_users_with_similar_words::<I, A, E>)
        )
        .route("/api/similar/{user_id}/recommend",
            get(get_recommended_words_for_user::<I, A, E>)
        )
        .route("/api/stamps",
            get(stamps_get::<I, A, E>).post(stamps_post::<I, A, E>)
        )
        .route("/api/trend/day/today",
            get(get_today_trending_words::<I, A, E>)
        )
        .route("/api/trend/day/{day}",
            get(get_trending_words_for_day::<I, A, E>)
        )
        .route("/api/trend/month/{month}",
            get(get_trending_words_for_month::<I, A, E>)
        )
        .route("/api/trend/year/{year}",
            get(get_trending_words_for_year::<I, A, E>)
        )
        .route("/api/words",
            get(words_get::<I, A, E>).post(words_post::<I, A, E>)
        )
        .route("/api/words/me",
            get(words_me_get::<I, A, E>)
        )
        .route("/api/words/{word_id}",
            delete(words_word_id_delete::<I, A, E>).put(words_word_id_put::<I, A, E>)
        )
        .with_state(api_impl)
}


#[tracing::instrument(skip_all)]
fn get_recommended_words_for_user_validation(
  path_params: models::GetRecommendedWordsForUserPathParams,
) -> std::result::Result<(
  models::GetRecommendedWordsForUserPathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}
/// GetRecommendedWordsForUser - GET /api/similar/{userId}/recommend
#[tracing::instrument(skip_all)]
async fn get_recommended_words_for_user<I, A, E>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::GetRecommendedWordsForUserPathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::similar::Similar<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    get_recommended_words_for_user_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().get_recommended_words_for_user(
      &method,
      &host,
      &cookies,
        &path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::similar::GetRecommendedWordsForUserResponse::Status200_OK
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn get_users_with_similar_words_validation(
  path_params: models::GetUsersWithSimilarWordsPathParams,
) -> std::result::Result<(
  models::GetUsersWithSimilarWordsPathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}
/// GetUsersWithSimilarWords - GET /api/similar/{userId}
#[tracing::instrument(skip_all)]
async fn get_users_with_similar_words<I, A, E>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::GetUsersWithSimilarWordsPathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::similar::Similar<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    get_users_with_similar_words_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().get_users_with_similar_words(
      &method,
      &host,
      &cookies,
        &path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::similar::GetUsersWithSimilarWordsResponse::Status200_OK
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn stamps_get_validation(
  query_params: models::StampsGetQueryParams,
) -> std::result::Result<(
  models::StampsGetQueryParams,
), ValidationErrors>
{
  query_params.validate()?;

Ok((
  query_params,
))
}
/// StampsGet - GET /api/stamps
#[tracing::instrument(skip_all)]
async fn stamps_get<I, A, E>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Query(query_params): Query<models::StampsGetQueryParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::stamps::Stamps<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    stamps_get_validation(
        query_params,
    )
  ).await.unwrap();

  let Ok((
    query_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().stamps_get(
      &method,
      &host,
      &cookies,
        &query_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::stamps::StampsGetResponse::Status200_SuccessfulRetrieval
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct StampsPostBodyValidator<'a> {
            #[validate(nested)]
          body: &'a models::NewStamp,
    }


#[tracing::instrument(skip_all)]
fn stamps_post_validation(
  header_params: models::StampsPostHeaderParams,
        body: models::NewStamp,
) -> std::result::Result<(
  models::StampsPostHeaderParams,
        models::NewStamp,
), ValidationErrors>
{
  header_params.validate()?;
              let b = StampsPostBodyValidator { body: &body };
              b.validate()?;

Ok((
  header_params,
    body,
))
}
/// StampsPost - POST /api/stamps
#[tracing::instrument(skip_all)]
async fn stamps_post<I, A, E>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  headers: HeaderMap,
 State(api_impl): State<I>,
          Json(body): Json<models::NewStamp>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::stamps::Stamps<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {

    // Header parameters
    let header_params = {
                let header_x_forwarded_user = headers.get(HeaderName::from_static("x-forwarded-user"));

                let header_x_forwarded_user = match header_x_forwarded_user {
                    Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
                        Ok(result) =>
                            result.0,
                        Err(err) => {
                            return Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Invalid header X-Forwarded-User - {}", err))).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });

                        },
                    },
                    None => {
                        return Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from("Missing required header X-Forwarded-User")).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });
                    }
                };

       models::StampsPostHeaderParams {
          x_forwarded_user: header_x_forwarded_user,
       }
  };


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    stamps_post_validation(
        header_params,
          body,
    )
  ).await.unwrap();

  let Ok((
    header_params,
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().stamps_post(
      &method,
      &host,
      &cookies,
        &header_params,
              &body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::stamps::StampsPostResponse::Status200_SuccessfulRegistration
                                                => {
                                                  let mut response = response.status(200);
                                                  response.body(Body::empty())
                                                },
                                                apis::stamps::StampsPostResponse::Status400_InvalidInput
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(why) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn get_today_trending_words_validation(
  query_params: models::GetTodayTrendingWordsQueryParams,
) -> std::result::Result<(
  models::GetTodayTrendingWordsQueryParams,
), ValidationErrors>
{
  query_params.validate()?;

Ok((
  query_params,
))
}
/// GetTodayTrendingWords - GET /api/trend/day/today
#[tracing::instrument(skip_all)]
async fn get_today_trending_words<I, A, E>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Query(query_params): Query<models::GetTodayTrendingWordsQueryParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::trend::Trend<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    get_today_trending_words_validation(
        query_params,
    )
  ).await.unwrap();

  let Ok((
    query_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().get_today_trending_words(
      &method,
      &host,
      &cookies,
        &query_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::trend::GetTodayTrendingWordsResponse::Status200_OK
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn get_trending_words_for_day_validation(
  path_params: models::GetTrendingWordsForDayPathParams,
  query_params: models::GetTrendingWordsForDayQueryParams,
) -> std::result::Result<(
  models::GetTrendingWordsForDayPathParams,
  models::GetTrendingWordsForDayQueryParams,
), ValidationErrors>
{
  path_params.validate()?;
  query_params.validate()?;

Ok((
  path_params,
  query_params,
))
}
/// GetTrendingWordsForDay - GET /api/trend/day/{day}
#[tracing::instrument(skip_all)]
async fn get_trending_words_for_day<I, A, E>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::GetTrendingWordsForDayPathParams>,
  Query(query_params): Query<models::GetTrendingWordsForDayQueryParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::trend::Trend<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    get_trending_words_for_day_validation(
        path_params,
        query_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
    query_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().get_trending_words_for_day(
      &method,
      &host,
      &cookies,
        &path_params,
        &query_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::trend::GetTrendingWordsForDayResponse::Status200_OK
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn get_trending_words_for_month_validation(
  path_params: models::GetTrendingWordsForMonthPathParams,
  query_params: models::GetTrendingWordsForMonthQueryParams,
) -> std::result::Result<(
  models::GetTrendingWordsForMonthPathParams,
  models::GetTrendingWordsForMonthQueryParams,
), ValidationErrors>
{
  path_params.validate()?;
  query_params.validate()?;

Ok((
  path_params,
  query_params,
))
}
/// GetTrendingWordsForMonth - GET /api/trend/month/{month}
#[tracing::instrument(skip_all)]
async fn get_trending_words_for_month<I, A, E>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::GetTrendingWordsForMonthPathParams>,
  Query(query_params): Query<models::GetTrendingWordsForMonthQueryParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::trend::Trend<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    get_trending_words_for_month_validation(
        path_params,
        query_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
    query_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().get_trending_words_for_month(
      &method,
      &host,
      &cookies,
        &path_params,
        &query_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::trend::GetTrendingWordsForMonthResponse::Status200_OK
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn get_trending_words_for_year_validation(
  path_params: models::GetTrendingWordsForYearPathParams,
  query_params: models::GetTrendingWordsForYearQueryParams,
) -> std::result::Result<(
  models::GetTrendingWordsForYearPathParams,
  models::GetTrendingWordsForYearQueryParams,
), ValidationErrors>
{
  path_params.validate()?;
  query_params.validate()?;

Ok((
  path_params,
  query_params,
))
}
/// GetTrendingWordsForYear - GET /api/trend/year/{year}
#[tracing::instrument(skip_all)]
async fn get_trending_words_for_year<I, A, E>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::GetTrendingWordsForYearPathParams>,
  Query(query_params): Query<models::GetTrendingWordsForYearQueryParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::trend::Trend<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    get_trending_words_for_year_validation(
        path_params,
        query_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
    query_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().get_trending_words_for_year(
      &method,
      &host,
      &cookies,
        &path_params,
        &query_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::trend::GetTrendingWordsForYearResponse::Status200_OK
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn words_get_validation(
  query_params: models::WordsGetQueryParams,
) -> std::result::Result<(
  models::WordsGetQueryParams,
), ValidationErrors>
{
  query_params.validate()?;

Ok((
  query_params,
))
}
/// WordsGet - GET /api/words
#[tracing::instrument(skip_all)]
async fn words_get<I, A, E>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Query(query_params): Query<models::WordsGetQueryParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::words::Words<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    words_get_validation(
        query_params,
    )
  ).await.unwrap();

  let Ok((
    query_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().words_get(
      &method,
      &host,
      &cookies,
        &query_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::words::WordsGetResponse::Status200_SuccessfulRetrieval
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::words::WordsGetResponse::Status404_NotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(why) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn words_me_get_validation(
  header_params: models::WordsMeGetHeaderParams,
) -> std::result::Result<(
  models::WordsMeGetHeaderParams,
), ValidationErrors>
{
  header_params.validate()?;

Ok((
  header_params,
))
}
/// WordsMeGet - GET /api/words/me
#[tracing::instrument(skip_all)]
async fn words_me_get<I, A, E>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  headers: HeaderMap,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::words::Words<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {

    // Header parameters
    let header_params = {
                let header_x_forwarded_user = headers.get(HeaderName::from_static("x-forwarded-user"));

                let header_x_forwarded_user = match header_x_forwarded_user {
                    Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
                        Ok(result) =>
                            result.0,
                        Err(err) => {
                            return Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Invalid header X-Forwarded-User - {}", err))).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });

                        },
                    },
                    None => {
                        return Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from("Missing required header X-Forwarded-User")).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });
                    }
                };

       models::WordsMeGetHeaderParams {
          x_forwarded_user: header_x_forwarded_user,
       }
  };


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    words_me_get_validation(
        header_params,
    )
  ).await.unwrap();

  let Ok((
    header_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().words_me_get(
      &method,
      &host,
      &cookies,
        &header_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::words::WordsMeGetResponse::Status200_SuccessfulRetrieval
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::words::WordsMeGetResponse::Status400_InvalidInput
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(why) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct WordsPostBodyValidator<'a> {
            #[validate(nested)]
          body: &'a models::NewWord,
    }


#[tracing::instrument(skip_all)]
fn words_post_validation(
  header_params: models::WordsPostHeaderParams,
        body: models::NewWord,
) -> std::result::Result<(
  models::WordsPostHeaderParams,
        models::NewWord,
), ValidationErrors>
{
  header_params.validate()?;
              let b = WordsPostBodyValidator { body: &body };
              b.validate()?;

Ok((
  header_params,
    body,
))
}
/// WordsPost - POST /api/words
#[tracing::instrument(skip_all)]
async fn words_post<I, A, E>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  headers: HeaderMap,
 State(api_impl): State<I>,
          Json(body): Json<models::NewWord>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::words::Words<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {

    // Header parameters
    let header_params = {
                let header_x_forwarded_user = headers.get(HeaderName::from_static("x-forwarded-user"));

                let header_x_forwarded_user = match header_x_forwarded_user {
                    Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
                        Ok(result) =>
                            result.0,
                        Err(err) => {
                            return Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Invalid header X-Forwarded-User - {}", err))).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });

                        },
                    },
                    None => {
                        return Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from("Missing required header X-Forwarded-User")).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });
                    }
                };

       models::WordsPostHeaderParams {
          x_forwarded_user: header_x_forwarded_user,
       }
  };


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    words_post_validation(
        header_params,
          body,
    )
  ).await.unwrap();

  let Ok((
    header_params,
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().words_post(
      &method,
      &host,
      &cookies,
        &header_params,
              &body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::words::WordsPostResponse::Status200_SuccessfulRegistration
                                                => {
                                                  let mut response = response.status(200);
                                                  response.body(Body::empty())
                                                },
                                                apis::words::WordsPostResponse::Status400_InvalidInput
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(why) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn words_word_id_delete_validation(
  header_params: models::WordsWordIdDeleteHeaderParams,
  path_params: models::WordsWordIdDeletePathParams,
) -> std::result::Result<(
  models::WordsWordIdDeleteHeaderParams,
  models::WordsWordIdDeletePathParams,
), ValidationErrors>
{
  header_params.validate()?;
  path_params.validate()?;

Ok((
  header_params,
  path_params,
))
}
/// WordsWordIdDelete - DELETE /api/words/{wordId}
#[tracing::instrument(skip_all)]
async fn words_word_id_delete<I, A, E>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  headers: HeaderMap,
  Path(path_params): Path<models::WordsWordIdDeletePathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::words::Words<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {

    // Header parameters
    let header_params = {
                let header_x_forwarded_user = headers.get(HeaderName::from_static("x-forwarded-user"));

                let header_x_forwarded_user = match header_x_forwarded_user {
                    Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
                        Ok(result) =>
                            result.0,
                        Err(err) => {
                            return Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Invalid header X-Forwarded-User - {}", err))).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });

                        },
                    },
                    None => {
                        return Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from("Missing required header X-Forwarded-User")).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });
                    }
                };

       models::WordsWordIdDeleteHeaderParams {
          x_forwarded_user: header_x_forwarded_user,
       }
  };


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    words_word_id_delete_validation(
        header_params,
        path_params,
    )
  ).await.unwrap();

  let Ok((
    header_params,
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().words_word_id_delete(
      &method,
      &host,
      &cookies,
        &header_params,
        &path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::words::WordsWordIdDeleteResponse::Status200_SuccessfulDeletion
                                                => {
                                                  let mut response = response.status(200);
                                                  response.body(Body::empty())
                                                },
                                                apis::words::WordsWordIdDeleteResponse::Status404_NotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(why) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct WordsWordIdPutBodyValidator<'a> {
            #[validate(nested)]
          body: &'a models::ExcludedUsers,
    }


#[tracing::instrument(skip_all)]
fn words_word_id_put_validation(
  header_params: models::WordsWordIdPutHeaderParams,
  path_params: models::WordsWordIdPutPathParams,
        body: models::ExcludedUsers,
) -> std::result::Result<(
  models::WordsWordIdPutHeaderParams,
  models::WordsWordIdPutPathParams,
        models::ExcludedUsers,
), ValidationErrors>
{
  header_params.validate()?;
  path_params.validate()?;
              let b = WordsWordIdPutBodyValidator { body: &body };
              b.validate()?;

Ok((
  header_params,
  path_params,
    body,
))
}
/// WordsWordIdPut - PUT /api/words/{wordId}
#[tracing::instrument(skip_all)]
async fn words_word_id_put<I, A, E>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  headers: HeaderMap,
  Path(path_params): Path<models::WordsWordIdPutPathParams>,
 State(api_impl): State<I>,
          Json(body): Json<models::ExcludedUsers>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::words::Words<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {

    // Header parameters
    let header_params = {
                let header_x_forwarded_user = headers.get(HeaderName::from_static("x-forwarded-user"));

                let header_x_forwarded_user = match header_x_forwarded_user {
                    Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
                        Ok(result) =>
                            result.0,
                        Err(err) => {
                            return Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Invalid header X-Forwarded-User - {}", err))).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });

                        },
                    },
                    None => {
                        return Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from("Missing required header X-Forwarded-User")).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });
                    }
                };

       models::WordsWordIdPutHeaderParams {
          x_forwarded_user: header_x_forwarded_user,
       }
  };


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    words_word_id_put_validation(
        header_params,
        path_params,
          body,
    )
  ).await.unwrap();

  let Ok((
    header_params,
    path_params,
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().words_word_id_put(
      &method,
      &host,
      &cookies,
        &header_params,
        &path_params,
              &body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::words::WordsWordIdPutResponse::Status200_SuccessfulEdit
                                                => {
                                                  let mut response = response.status(200);
                                                  response.body(Body::empty())
                                                },
                                                apis::words::WordsWordIdPutResponse::Status404_NotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(why) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

