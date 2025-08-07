use crate::{json::Json, state::StateTrait};
use axum::{
    body::Bytes,
    extract::State,
    response::{IntoResponse, Response},
};
use iam_common::{
    error::{oauth::OAuthError, Error},
    keys::jwt::Claims,
    password,
};
use iam_entity::users;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    access_token: String,
    token_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_in: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refresh_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<String>,
}

pub async fn token<S: StateTrait>(
    State(state): State<S>,
    body: Bytes,
) -> Result<Json<TokenResponse>, Response> {
    #[derive(Deserialize)]
    pub struct GrantTypeRequest<'a> {
        #[serde(borrow)]
        grant_type: Cow<'a, str>,
    }

    let request = serde_urlencoded::from_bytes::<GrantTypeRequest<'_>>(&body)
        .map_err(|_| OAuthError::invalid_request().into_response())?;

    if &request.grant_type == "password" {
        let request =
            serde_urlencoded::from_bytes::<ResourceOwnerPasswordCredentialsGrantRequest<'_>>(&body)
                .map_err(|_| OAuthError::invalid_request().into_response())?;

        return resource_owner_password_credentials_grant(&state, request).await;
    }

    Err(OAuthError::unsupported_grant_type().into_response())
}

#[derive(Debug, Deserialize)]
struct ResourceOwnerPasswordCredentialsGrantRequest<'a> {
    #[serde(borrow)]
    username: Cow<'a, str>,
    #[serde(borrow)]
    password: Cow<'a, str>,
}

async fn resource_owner_password_credentials_grant<S: StateTrait>(
    state: &S,
    request: ResourceOwnerPasswordCredentialsGrantRequest<'_>,
) -> Result<Json<TokenResponse>, Response> {
    let res = users::Entity::find()
        .filter(users::Column::Email.eq(request.username.as_ref()))
        .one(state.db())
        .await
        .map_err(|err| Error::from(err).into_response())?;

    let Some(res) = res else {
        // Still compute hash if the user is not in the database to avoid timing attacks
        _ = password::hash(&request.password);
        return Err(OAuthError::invalid_grant().into_response());
    };

    let (valid, rehash) = password::validate(&res.password, request.password.as_ref())
        .map_err(|_| OAuthError::invalid_grant().into_response())?;

    if let Some(Ok(hash)) = rehash {
        let mut action: users::ActiveModel = res.clone().into();
        action.password = ActiveValue::Set(hash);

        action
            .update(state.db())
            .await
            .map_err(|err| Error::from(err).into_response())?;
    }

    if !valid {
        return Err(OAuthError::invalid_grant().into_response());
    }

    crate::audit!(action = "login", user = res.id);

    let token = state.key_manager().jwt().encode(&Claims::new(res.id));

    Ok(Json(TokenResponse {
        access_token: token,
        token_type: String::from("bearer"),
        refresh_token: None,
        expires_in: None,
        scope: None,
    }))
}
