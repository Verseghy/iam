use crate::{json::Json, json::ValidatedJson, state::StateTrait, utils::DatabaseErrorType};
use axum::{extract::State, http::StatusCode};
use iam_common::{
    error::{self, Result},
    Id,
};
use iam_entity::users;
use sea_orm::{EntityTrait, Set};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct Request {
    #[validate(length(max = 256))]
    name: String,
    #[validate(email, length(max = 256))]
    email: String,
    #[validate(length(max = 256))]
    password: String,
}

#[derive(Serialize, Debug)]
pub struct Response {
    id: Id,
}

pub async fn register<S: StateTrait>(
    State(state): State<S>,
    ValidatedJson(req): ValidatedJson<Request>,
) -> Result<(StatusCode, Json<Response>)> {
    let id = Id::new_user();

    let model = users::ActiveModel {
        id: Set(id.to_string()),
        password: Set(iam_common::password::hash(&req.password)?),
        name: Set(req.name),
        email: Set(req.email),
        ..Default::default()
    };

    let result = users::Entity::insert(model).exec(state.db()).await;

    if let Err(err) = result {
        if err.is_duplicate_entry() {
            return Err(error::EMAIL_ALREADY_REGISTERED);
        }

        Err(err)?;
    }

    Ok((StatusCode::CREATED, Json(Response { id })))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        state::mock::MockState,
        utils::testing::{assert_error, body_to_json, json_body},
    };
    use axum::{
        handler::Handler,
        http::{self, Request, StatusCode},
    };
    use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};
    use tower::ServiceExt;

    #[tokio::test]
    async fn correct() {
        let state = MockState::builder()
            .db(
                MockDatabase::new(DatabaseBackend::MySql).append_exec_results(vec![
                    MockExecResult {
                        last_insert_id: 0,
                        rows_affected: 1,
                    },
                ]),
            )
            .build();
        let app = register::<MockState>.with_state(state);

        let res = app
            .oneshot(
                Request::post("/")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(json_body!({
                        "name": "test",
                        "email": "test@test.test",
                        "password": "test",
                    }))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::CREATED);

        let body = body_to_json(res.into_body()).await;
        assert!(body.is_object());
        assert!(body["id"].is_string());
        assert!(body["id"].as_str().unwrap().starts_with("UserID-"));
    }

    #[tokio::test]
    async fn invalid_email() {
        let state = MockState::empty();
        let app = register::<MockState>.with_state(state);

        let res = app
            .oneshot(
                Request::post("/")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(json_body!({
                        "name": "test",
                        "email": "invalid_email",
                        "password": "test",
                    }))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_error!(res, error::JSON_VALIDATE_INVALID);
    }

    #[tokio::test]
    async fn long_name() {
        let state = MockState::empty();
        let app = register::<MockState>.with_state(state);

        let res = app
            .oneshot(
                Request::post("/")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(json_body!({
                        "name": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                        "email": "test@test.test",
                        "password": "test",
                    }))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_error!(res, error::JSON_VALIDATE_INVALID);
    }

    #[tokio::test]
    async fn long_email() {
        let state = MockState::empty();
        let app = register::<MockState>.with_state(state);

        let res = app
            .oneshot(
                Request::post("/")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(json_body!({
                        "name": "test",
                        "email": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa@test.test",
                        "password": "test",
                    }))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_error!(res, error::JSON_VALIDATE_INVALID);
    }

    #[tokio::test]
    async fn long_password() {
        let state = MockState::empty();
        let app = register::<MockState>.with_state(state);

        let res = app
            .oneshot(
                Request::post("/")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(json_body!({
                        "name": "test",
                        "email": "test@test.test",
                        "password": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                    }))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_error!(res, error::JSON_VALIDATE_INVALID);
    }
}
