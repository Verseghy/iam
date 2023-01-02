use crate::{json::Json, json::ValidatedJson, shared::SharedTrait, utils::DatabaseErrorType};
use axum::{http::StatusCode, Extension};
use common::{
    error::{self, Result},
    Id,
};
use entity::users;
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

pub async fn register<S: SharedTrait>(
    Extension(shared): Extension<S>,
    ValidatedJson(req): ValidatedJson<Request>,
) -> Result<(StatusCode, Json<Response>)> {
    let id = Id::new_user();

    let model = users::ActiveModel {
        id: Set(id.to_string()),
        password: Set(common::password::hash(&req.password)?),
        name: Set(req.name),
        email: Set(req.email),
        ..Default::default()
    };

    let result = users::Entity::insert(model).exec(shared.db()).await;

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
        shared::mock::MockShared,
        utils::testing::{body_to_json, json_body},
    };
    use axum::{
        handler::HandlerWithoutStateExt,
        http::{self, Request, StatusCode},
    };
    use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};
    use serde_json::json;
    use tower::ServiceExt;

    #[tokio::test]
    async fn correct() {
        let app = register::<MockShared>.into_service();
        let shared = MockShared::builder()
            .db(
                MockDatabase::new(DatabaseBackend::MySql).append_exec_results(vec![
                    MockExecResult {
                        last_insert_id: 0,
                        rows_affected: 1,
                    },
                ]),
            )
            .build();

        let res = app
            .oneshot(
                Request::post("/")
                    .extension(shared)
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
        let app = register::<MockShared>.into_service();

        let res = app
            .oneshot(
                Request::post("/")
                    .extension(MockShared::empty())
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

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let body = body_to_json(res.into_body()).await;
        assert_eq!(body, json!({"error": "invalid data"}));
    }

    #[tokio::test]
    async fn long_name() {
        let app = register::<MockShared>.into_service();

        let res = app
            .oneshot(
                Request::post("/")
                    .extension(MockShared::empty())
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

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let body = body_to_json(res.into_body()).await;
        assert_eq!(body, json!({"error": "invalid data"}));
    }

    #[tokio::test]
    async fn long_email() {
        let app = register::<MockShared>.into_service();

        let res = app
            .oneshot(
                Request::post("/")
                    .extension(MockShared::empty())
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

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let body = body_to_json(res.into_body()).await;
        assert_eq!(body, json!({"error": "invalid data"}));
    }

    #[tokio::test]
    async fn long_password() {
        let app = register::<MockShared>.into_service();

        let res = app
            .oneshot(
                Request::post("/")
                    .extension(MockShared::empty())
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

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let body = body_to_json(res.into_body()).await;
        assert_eq!(body, json!({"error": "invalid data"}));
    }
}
