use core::fmt;
use std::error::Error;

use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use shtml::{html, Component};

#[derive(Debug)]
pub enum ShatError {
    NotFound,
    Unauthorized,
    BadRequest,
    InternalError,
}

impl fmt::Display for ShatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShatError::NotFound => write!(f, "Not found"),
            ShatError::Unauthorized => write!(f, "Unauthenticated"),
            ShatError::BadRequest => write!(f, "Bad request"),
            ShatError::InternalError => write!(f, "Internal error"),
        }
    }
}

impl Error for ShatError {}

impl IntoResponse for ShatError {
    fn into_response(self) -> Response<Body> {
        let (status, component) = match self {
            ShatError::NotFound => (
                StatusCode::NOT_FOUND,
                html! {
                    404 not found
                },
            ),
            ShatError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                html! {
                    401 unauthorized
                },
            ),
            ShatError::BadRequest => (
                StatusCode::BAD_REQUEST,
                html! {
                    400 bad request
                },
            ),
            ShatError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                html! {
                    500 internal server error
                },
            ),
        };

        let body = Body::from(component.to_string());

        Response::builder().status(status).body(body).unwrap()
    }
}
