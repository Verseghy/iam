use crate::{
    json::Json,
    token::{self, JwtTrait},
    utils::Error,
    SharedTrait,
};
use axum::Extension;
use entity::apps;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Request {
    token: String,
}

#[derive(Serialize, Debug)]
pub struct Response {
    token: String,
}

pub async fn login_app<S: SharedTrait>(
    Extension(shared): Extension<S>,
    Json(request): Json<Request>,
) -> Result<Json<Response>, Error> {
    let (id, password) = parse_token(&request.token)?;

    let res = apps::Entity::find_by_id(id.clone())
        .one(shared.db())
        .await?
        .ok_or_else(|| Error::not_found("invalid token"))?;

    let (valid, _) =
        common::password::validate(&res.password, &password).map_err(Error::internal)?;

    if !valid {
        return Err(Error::bad_request("invalid token"));
    }

    let claims = token::Claims {
        subject: id,
        ..Default::default()
    };

    let token = shared.jwt().encode(&claims).map_err(Error::internal)?;

    Ok(Json(Response { token }))
}

fn parse_token(token: &str) -> Result<(String, String), Error> {
    let decoded = base64::decode(token).map_err(|_| Error::bad_request("invalid token"))?;
    let decoded_string =
        String::from_utf8(decoded).map_err(|_| Error::bad_request("invalid token"))?;

    let (id, password) = decoded_string
        .split_once(':')
        .ok_or_else(|| Error::bad_request("invalid token"))?;

    Ok((id.into(), password.into()))
}
