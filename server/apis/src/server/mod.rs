use std::collections::HashMap;

use axum::{body::Body, extract::*, response::Response, routing::*};
use axum_extra::extract::{CookieJar, Host, Query as QueryExtra};
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
    A: apis::me::Me<E> + apis::stamps::Stamps<E> + apis::words::Words<E> + apis::me_stamps::MeStamps<E> + apis::me_words::MeWords<E> + Send + Sync + 'static,
    E: std::fmt::Debug + Send + Sync + 'static,
    
{
    // build our application with a route
    Router::new()
        .route("/api/me",
            get(me_get::<I, A, E>)
        )
        .route("/api/me/stamps",
            get(me_stamps_get::<I, A, E>)
        )
        .route("/api/me/words",
            get(me_words_get::<I, A, E>)
        )
        .route("/api/stamps",
            get(stamps_get::<I, A, E>).post(stamps_post::<I, A, E>)
        )
        .route("/api/stamps/{stamp_id}",
            delete(stamps_stamp_id_delete::<I, A, E>)
        )
        .route("/api/stamps/{stamp_id}/exclusions",
            put(stamps_stamp_id_exclusions_put::<I, A, E>)
        )
        .route("/api/words",
            post(words_post::<I, A, E>)
        )
        .route("/api/words/{word_id}",
            delete(words_word_id_delete::<I, A, E>)
        )
        .route("/api/words/{word_id}/exclusions",
            put(words_word_id_exclusions_put::<I, A, E>)
        )
        .with_state(api_impl)
}


#[tracing::instrument(skip_all)]
fn me_get_validation(
  header_params: models::MeGetHeaderParams,
) -> std::result::Result<(
  models::MeGetHeaderParams,
), ValidationErrors>
{
  header_params.validate()?;

Ok((
  header_params,
))
}
/// MeGet - GET /api/me
#[tracing::instrument(skip_all)]
async fn me_get<I, A, E>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  headers: HeaderMap,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::me::Me<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {

    // Header parameters
    let header_params = {
                let header_x_forwarded_user = headers.get(HeaderName::from_static("x-forwarded-user"));

                let header_x_forwarded_user = match header_x_forwarded_user {
                    Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
                        Ok(result) =>
                            Some(result.0),
                        Err(err) => {
                            return Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Invalid header X-Forwarded-User - {}", err))).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });

                        },
                    },
                    None => {
                        None
                    }
                };

       models::MeGetHeaderParams {
          x_forwarded_user: header_x_forwarded_user,
       }
  };


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    me_get_validation(
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

  let result = api_impl.as_ref().me_get(
      &method,
      &host,
      &cookies,
        &header_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::me::MeGetResponse::Status200_SuccessfulRetrieval
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
                                                apis::me::MeGetResponse::Status404_NotFound
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
fn stamps_get_validation(
  header_params: models::StampsGetHeaderParams,
) -> std::result::Result<(
  models::StampsGetHeaderParams,
), ValidationErrors>
{
  header_params.validate()?;

Ok((
  header_params,
))
}
/// StampsGet - GET /api/stamps
#[tracing::instrument(skip_all)]
async fn stamps_get<I, A, E>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  headers: HeaderMap,
 State(api_impl): State<I>,
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
                            Some(result.0),
                        Err(err) => {
                            return Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Invalid header X-Forwarded-User - {}", err))).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });

                        },
                    },
                    None => {
                        None
                    }
                };

       models::StampsGetHeaderParams {
          x_forwarded_user: header_x_forwarded_user,
       }
  };


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    stamps_get_validation(
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

  let result = api_impl.as_ref().stamps_get(
      &method,
      &host,
      &cookies,
        &header_params,
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
                            Some(result.0),
                        Err(err) => {
                            return Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Invalid header X-Forwarded-User - {}", err))).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });

                        },
                    },
                    None => {
                        None
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
                                                apis::stamps::StampsPostResponse::Status201_SuccessfulRegistration
                                                => {
                                                  let mut response = response.status(201);
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
fn stamps_stamp_id_delete_validation(
  header_params: models::StampsStampIdDeleteHeaderParams,
  path_params: models::StampsStampIdDeletePathParams,
) -> std::result::Result<(
  models::StampsStampIdDeleteHeaderParams,
  models::StampsStampIdDeletePathParams,
), ValidationErrors>
{
  header_params.validate()?;
  path_params.validate()?;

Ok((
  header_params,
  path_params,
))
}
/// StampsStampIdDelete - DELETE /api/stamps/{stampId}
#[tracing::instrument(skip_all)]
async fn stamps_stamp_id_delete<I, A, E>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  headers: HeaderMap,
  Path(path_params): Path<models::StampsStampIdDeletePathParams>,
 State(api_impl): State<I>,
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
                            Some(result.0),
                        Err(err) => {
                            return Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Invalid header X-Forwarded-User - {}", err))).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });

                        },
                    },
                    None => {
                        None
                    }
                };

       models::StampsStampIdDeleteHeaderParams {
          x_forwarded_user: header_x_forwarded_user,
       }
  };


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    stamps_stamp_id_delete_validation(
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

  let result = api_impl.as_ref().stamps_stamp_id_delete(
      &method,
      &host,
      &cookies,
        &header_params,
        &path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::stamps::StampsStampIdDeleteResponse::Status204_SuccessfulDeletion
                                                => {
                                                  let mut response = response.status(204);
                                                  response.body(Body::empty())
                                                },
                                                apis::stamps::StampsStampIdDeleteResponse::Status404_NotFound
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
    struct StampsStampIdExclusionsPutBodyValidator<'a> {
            #[validate(nested)]
          body: &'a models::ExcludedUsers,
    }


#[tracing::instrument(skip_all)]
fn stamps_stamp_id_exclusions_put_validation(
  header_params: models::StampsStampIdExclusionsPutHeaderParams,
  path_params: models::StampsStampIdExclusionsPutPathParams,
        body: models::ExcludedUsers,
) -> std::result::Result<(
  models::StampsStampIdExclusionsPutHeaderParams,
  models::StampsStampIdExclusionsPutPathParams,
        models::ExcludedUsers,
), ValidationErrors>
{
  header_params.validate()?;
  path_params.validate()?;
              let b = StampsStampIdExclusionsPutBodyValidator { body: &body };
              b.validate()?;

Ok((
  header_params,
  path_params,
    body,
))
}
/// StampsStampIdExclusionsPut - PUT /api/stamps/{stampId}/exclusions
#[tracing::instrument(skip_all)]
async fn stamps_stamp_id_exclusions_put<I, A, E>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  headers: HeaderMap,
  Path(path_params): Path<models::StampsStampIdExclusionsPutPathParams>,
 State(api_impl): State<I>,
          Json(body): Json<models::ExcludedUsers>,
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
                            Some(result.0),
                        Err(err) => {
                            return Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Invalid header X-Forwarded-User - {}", err))).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });

                        },
                    },
                    None => {
                        None
                    }
                };

       models::StampsStampIdExclusionsPutHeaderParams {
          x_forwarded_user: header_x_forwarded_user,
       }
  };


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    stamps_stamp_id_exclusions_put_validation(
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

  let result = api_impl.as_ref().stamps_stamp_id_exclusions_put(
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
                                                apis::stamps::StampsStampIdExclusionsPutResponse::Status200_SuccessfulEdit
                                                => {
                                                  let mut response = response.status(200);
                                                  response.body(Body::empty())
                                                },
                                                apis::stamps::StampsStampIdExclusionsPutResponse::Status404_NotFound
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
                            Some(result.0),
                        Err(err) => {
                            return Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Invalid header X-Forwarded-User - {}", err))).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });

                        },
                    },
                    None => {
                        None
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
                                                apis::words::WordsPostResponse::Status201_SuccessfulRegistration
                                                => {
                                                  let mut response = response.status(201);
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
                            Some(result.0),
                        Err(err) => {
                            return Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Invalid header X-Forwarded-User - {}", err))).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });

                        },
                    },
                    None => {
                        None
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
                                                apis::words::WordsWordIdDeleteResponse::Status204_SuccessfulDeletion
                                                => {
                                                  let mut response = response.status(204);
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
    struct WordsWordIdExclusionsPutBodyValidator<'a> {
            #[validate(nested)]
          body: &'a models::ExcludedUsers,
    }


#[tracing::instrument(skip_all)]
fn words_word_id_exclusions_put_validation(
  header_params: models::WordsWordIdExclusionsPutHeaderParams,
  path_params: models::WordsWordIdExclusionsPutPathParams,
        body: models::ExcludedUsers,
) -> std::result::Result<(
  models::WordsWordIdExclusionsPutHeaderParams,
  models::WordsWordIdExclusionsPutPathParams,
        models::ExcludedUsers,
), ValidationErrors>
{
  header_params.validate()?;
  path_params.validate()?;
              let b = WordsWordIdExclusionsPutBodyValidator { body: &body };
              b.validate()?;

Ok((
  header_params,
  path_params,
    body,
))
}
/// WordsWordIdExclusionsPut - PUT /api/words/{wordId}/exclusions
#[tracing::instrument(skip_all)]
async fn words_word_id_exclusions_put<I, A, E>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  headers: HeaderMap,
  Path(path_params): Path<models::WordsWordIdExclusionsPutPathParams>,
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
                            Some(result.0),
                        Err(err) => {
                            return Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Invalid header X-Forwarded-User - {}", err))).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });

                        },
                    },
                    None => {
                        None
                    }
                };

       models::WordsWordIdExclusionsPutHeaderParams {
          x_forwarded_user: header_x_forwarded_user,
       }
  };


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    words_word_id_exclusions_put_validation(
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

  let result = api_impl.as_ref().words_word_id_exclusions_put(
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
                                                apis::words::WordsWordIdExclusionsPutResponse::Status200_SuccessfulEdit
                                                => {
                                                  let mut response = response.status(200);
                                                  response.body(Body::empty())
                                                },
                                                apis::words::WordsWordIdExclusionsPutResponse::Status404_NotFound
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
fn me_stamps_get_validation(
  header_params: models::MeStampsGetHeaderParams,
) -> std::result::Result<(
  models::MeStampsGetHeaderParams,
), ValidationErrors>
{
  header_params.validate()?;

Ok((
  header_params,
))
}
/// MeStampsGet - GET /api/me/stamps
#[tracing::instrument(skip_all)]
async fn me_stamps_get<I, A, E>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  headers: HeaderMap,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::me_stamps::MeStamps<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {

    // Header parameters
    let header_params = {
                let header_x_forwarded_user = headers.get(HeaderName::from_static("x-forwarded-user"));

                let header_x_forwarded_user = match header_x_forwarded_user {
                    Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
                        Ok(result) =>
                            Some(result.0),
                        Err(err) => {
                            return Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Invalid header X-Forwarded-User - {}", err))).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });

                        },
                    },
                    None => {
                        None
                    }
                };

       models::MeStampsGetHeaderParams {
          x_forwarded_user: header_x_forwarded_user,
       }
  };


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    me_stamps_get_validation(
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

  let result = api_impl.as_ref().me_stamps_get(
      &method,
      &host,
      &cookies,
        &header_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::me_stamps::MeStampsGetResponse::Status200_SuccessfulRetrieval
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
                                                apis::me_stamps::MeStampsGetResponse::Status404_NotFound
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
fn me_words_get_validation(
  header_params: models::MeWordsGetHeaderParams,
) -> std::result::Result<(
  models::MeWordsGetHeaderParams,
), ValidationErrors>
{
  header_params.validate()?;

Ok((
  header_params,
))
}
/// MeWordsGet - GET /api/me/words
#[tracing::instrument(skip_all)]
async fn me_words_get<I, A, E>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  headers: HeaderMap,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::me_words::MeWords<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
        {

    // Header parameters
    let header_params = {
                let header_x_forwarded_user = headers.get(HeaderName::from_static("x-forwarded-user"));

                let header_x_forwarded_user = match header_x_forwarded_user {
                    Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
                        Ok(result) =>
                            Some(result.0),
                        Err(err) => {
                            return Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Invalid header X-Forwarded-User - {}", err))).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });

                        },
                    },
                    None => {
                        None
                    }
                };

       models::MeWordsGetHeaderParams {
          x_forwarded_user: header_x_forwarded_user,
       }
  };


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    me_words_get_validation(
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

  let result = api_impl.as_ref().me_words_get(
      &method,
      &host,
      &cookies,
        &header_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::me_words::MeWordsGetResponse::Status200_SuccessfulRetrieval
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
                                                apis::me_words::MeWordsGetResponse::Status404_NotFound
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

