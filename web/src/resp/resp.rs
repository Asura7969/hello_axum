use std::ops::{Deref, DerefMut};
use axum::http::{header, HeaderValue, StatusCode};
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use crate::resp;

#[derive(Serialize)]
#[cfg_attr(docsrs, doc(cfg(feature = "json")))]
pub struct RestJson<T> {
    code: u16,
    msg: String,
    data: Option<T>,
}

impl<T> RestJson<T>
where
    T: Serialize,
{

    pub fn new(code: u16, msg: String, data: Option<T>) -> Self {
        Self { code, msg, data }
    }

    pub fn ok(data: T) -> RestJson<T> {
        Self {code: StatusCode::OK.as_u16(), msg: "OK".to_string(), data: Some(data)}
    }

    pub fn err(code: u16, msg: String) -> Self {
        Self::new(code, msg, None)
    }
}

pub fn fail(code: StatusCode, msg: String) -> RestJson<String> {
    RestJson {code: code.as_u16(), msg, data: Some("".to_owned())}
}

impl<T> IntoResponse for RestJson<T>
    where
        T: Serialize,
{
    fn into_response(self) -> Response {
        match serde_json::to_vec(&self) {
            Ok(bytes) => (
                [(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()),
                )],
                bytes,
            ).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                [(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static(mime::TEXT_PLAIN_UTF_8.as_ref()),
                )],
                err.to_string(),
            ).into_response(),
        }
    }
}


