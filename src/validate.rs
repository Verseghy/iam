use axum::{
    async_trait,
    body::HttpBody,
    extract::{rejection::JsonRejection, FromRequest, Json, RequestParts},
    http::StatusCode,
    response::{IntoResponse, Response},
    BoxError,
};
use serde::de::DeserializeOwned;
use validator::{Validate, ValidationErrors};

pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, B> FromRequest<B> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    B: HttpBody + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = ValidateJsonRejection;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Json(json) = req
            .extract::<Json<T>>()
            .await
            .map_err(ValidateJsonRejection::Json)?;

        json.validate().map_err(ValidateJsonRejection::Validation)?;

        Ok(ValidatedJson(json))
    }
}

pub enum ValidateJsonRejection {
    Json(JsonRejection),
    Validation(ValidationErrors),
}

impl IntoResponse for ValidateJsonRejection {
    fn into_response(self) -> Response {
        match self {
            Self::Json(rejection) => rejection.into_response(),
            Self::Validation(_) => StatusCode::BAD_REQUEST.into_response(),
        }
    }
}
