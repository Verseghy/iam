use crate::{json::Json, shared::SharedTrait};
use axum::{extract::Path, Extension};
use common::error::{self, Result};
use entity::actions;
use sea_orm::entity::EntityTrait;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct GetResponse {
    id: String,
    name: String,
    secure: bool,
}

pub async fn get_action<S: SharedTrait>(
    Extension(shared): Extension<S>,
    Path(id): Path<String>,
) -> Result<Json<GetResponse>> {
    let res = actions::Entity::find_by_id(id)
        .one(shared.db())
        .await?
        .ok_or(error::ACTION_NOT_FOUND)?;

    Ok(Json(GetResponse {
        id: res.id,
        name: res.name,
        secure: res.secure,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        shared::mock::MockShared,
        utils::testing::{assert_error, body_to_json},
    };
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        routing::{get, Router},
    };
    use chrono::Utc;
    use sea_orm::{DatabaseBackend, MockDatabase};
    use serde_json::json;
    use tower::ServiceExt;

    #[tokio::test]
    async fn has_in_db() {
        let app = Router::new().route("/:id", get(get_action::<MockShared>));
        let shared = MockShared::builder()
            .db(
                MockDatabase::new(DatabaseBackend::MySql).append_query_results(vec![vec![
                    actions::Model {
                        id: "TestID-0".to_owned(),
                        name: "TestAction".to_owned(),
                        secure: false,
                        created_at: Utc::now().naive_utc(),
                        updated_at: Utc::now().naive_utc(),
                        deleted_at: None,
                    },
                ]]),
            )
            .build();

        let res = app
            .oneshot(
                Request::get("/TestID-0")
                    .extension(shared)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::OK);

        let body = body_to_json(res.into_body()).await;
        assert_eq!(
            body,
            json!({"id": "TestID-0", "name": "TestAction", "secure": false})
        );
    }

    #[tokio::test]
    async fn not_found() {
        let app = Router::new().route("/:id", get(get_action::<MockShared>));
        let shared = MockShared::builder()
            .db(MockDatabase::new(DatabaseBackend::MySql)
                .append_query_results::<actions::Model>(vec![vec![]]))
            .build();

        let res = app
            .oneshot(
                Request::get("/TestID-0")
                    .extension(shared)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_error!(res, error::ACTION_NOT_FOUND);
    }
}
