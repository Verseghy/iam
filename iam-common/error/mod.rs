mod constants;
pub mod oauth;

pub use constants::*;

use axum::{
    http::{HeaderValue, StatusCode, header},
    response::{IntoResponse, Response},
};
use bytes::{BufMut, BytesMut};
use sea_orm::DbErr;
use std::borrow::Cow;

#[doc(hidden)]
pub mod __macro_support {
    pub use axum::http::StatusCode;
    pub use iam_macros;
    pub use std::borrow::Cow;
}

#[derive(Debug)]
pub struct Error {
    status: StatusCode,
    code: &'static str,
    message: Cow<'static, str>,
}

pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    #[inline]
    pub const fn new(status: StatusCode, code: &'static str, message: Cow<'static, str>) -> Error {
        Self {
            status,
            code,
            message,
        }
    }

    #[inline]
    pub const fn code(&self) -> &'static str {
        self.code
    }

    #[inline]
    pub const fn status(&self) -> StatusCode {
        self.status
    }

    #[inline]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let mut buf = BytesMut::with_capacity(128).writer();

        serde_json::to_writer(
            &mut buf,
            &serde_json::json!({
                "code": self.code(),
                "error": self.message(),
            }),
        )
        .expect("failed to serialize error");

        let buf = buf.into_inner();
        let mut res = (self.status(), buf).into_response();

        res.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()),
        );

        res
    }
}

impl From<DbErr> for Error {
    #[inline]
    fn from(error: DbErr) -> Self {
        tracing::error!("database error: {:?}", error);
        constants::DATABASE_ERROR
    }
}

macro_rules! const_error {
    (
        #[error($code:literal, $status:ident)]
        #[message($msg:literal)]
        const $name:ident;
    ) => {
        $crate::error::__macro_support::iam_macros::error_code_to_ident!($code);
        pub const $name: $crate::error::Error = $crate::error::Error::new(
            $crate::error::__macro_support::StatusCode::$status,
            $code,
            $crate::error::__macro_support::Cow::Borrowed($msg),
        );
    };
}

pub(crate) use const_error;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_response_has_json_content_type() {
        let error = Error::new(StatusCode::OK, "", Cow::Borrowed(""));
        let response = error.into_response();
        let content_type = response.headers().get(header::CONTENT_TYPE);

        assert!(content_type.is_some(), "response");
        assert_eq!(content_type.unwrap(), "application/json");
    }
}
