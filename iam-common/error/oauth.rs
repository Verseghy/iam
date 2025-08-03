use axum::{
    http::{header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use bytes::{BufMut, BytesMut};
use serde::Serialize;

#[derive(Debug)]
#[non_exhaustive]
#[allow(unused)]
pub enum ErrorType {
    InvalidClient,
    InvalidGrant,
    InvalidRequest,
    InvalidScope,
    UnauthorizedClient,
    UnsupportedGrantType,
}

impl AsRef<str> for ErrorType {
    fn as_ref(&self) -> &str {
        match *self {
            ErrorType::InvalidClient => "invalid_client",
            ErrorType::InvalidGrant => "invalid_grant",
            ErrorType::InvalidRequest => "invalid_request",
            ErrorType::InvalidScope => "invalid_scope",
            ErrorType::UnauthorizedClient => "unauthorized_client",
            ErrorType::UnsupportedGrantType => "unsupported_grant_type",
        }
    }
}

impl Serialize for ErrorType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.as_ref())
    }
}

#[derive(Debug, Serialize)]
pub struct OAuthError {
    pub error: ErrorType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_uri: Option<String>,
}

impl OAuthError {
    pub const fn invalid_grant() -> Self {
        Self {
            error: ErrorType::InvalidGrant,
            error_description: None,
            error_uri: None,
        }
    }

    pub const fn invalid_request() -> Self {
        Self {
            error: ErrorType::InvalidRequest,
            error_description: None,
            error_uri: None,
        }
    }

    pub const fn unsupported_grant_type() -> Self {
        Self {
            error: ErrorType::UnsupportedGrantType,
            error_description: None,
            error_uri: None,
        }
    }
}

impl IntoResponse for OAuthError {
    fn into_response(self) -> Response {
        let mut buf = BytesMut::with_capacity(128).writer();

        serde_json::to_writer(&mut buf, &self).expect("failed to serialize error");

        let buf = buf.into_inner();
        let mut res = (StatusCode::BAD_REQUEST, buf).into_response();

        res.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()),
        );

        res
    }
}
