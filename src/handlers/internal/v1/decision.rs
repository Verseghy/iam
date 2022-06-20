use crate::{
    auth::permission::{self, CheckError},
    token::{get_claims, GetClaimsError},
};
use actix_web::{http::StatusCode, web, HttpRequest, HttpResponse, ResponseError};
use jsonwebtoken::DecodingKey;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct DecisionRequest {
    action_list: Vec<Action>,
}

#[derive(Deserialize, Debug)]
pub struct Action {
    name: String,
}

#[derive(Serialize, Debug)]
pub struct DecisionResponse {
    failed: String,
}

pub async fn decision(
    req: web::Json<DecisionRequest>,
    http_req: HttpRequest,
    db: web::Data<DatabaseConnection>,
    decoding_key: web::Data<DecodingKey>,
) -> Result<HttpResponse, DecisionError> {
    let claims = get_claims(http_req.headers(), &decoding_key)?;
    let permissions: Vec<&str> = req.action_list.iter().map(|x| x.name.as_str()).collect();

    match permission::check(&claims.subject, &permissions, db.get_ref()).await {
        Err(CheckError::NoPermission(failed)) => {
            Ok(HttpResponse::Forbidden().json(DecisionResponse { failed }))
        }
        Err(err) => Err(DecisionError::from(err)),
        Ok(_) => Ok(HttpResponse::new(StatusCode::NO_CONTENT)),
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DecisionError {
    #[error("get claims error: {0}")]
    GetClaimsError(#[from] GetClaimsError),
    #[error("check error: {0}")]
    CheckError(#[from] CheckError),
}

impl ResponseError for DecisionError {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::GetClaimsError(GetClaimsError::InvalidToken(_)) => StatusCode::UNAUTHORIZED,
            Self::GetClaimsError(GetClaimsError::NoAuthorizationHeader) => StatusCode::BAD_REQUEST,
            Self::GetClaimsError(GetClaimsError::NotUTF8Header(_)) => StatusCode::BAD_REQUEST,
            Self::GetClaimsError(GetClaimsError::NotBearerToken) => StatusCode::BAD_REQUEST,
            Self::CheckError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
