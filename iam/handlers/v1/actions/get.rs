use crate::{json::Json, state::StateTrait};
use axum::extract::{Path, State};
use iam_common::error::{self, Result};
use iam_entity::actions;
use sea_orm::entity::EntityTrait;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct GetResponse {
    id: String,
    name: String,
    secure: bool,
}

pub async fn get_action<S: StateTrait>(
    State(state): State<S>,
    Path(id): Path<String>,
) -> Result<Json<GetResponse>> {
    let res = actions::Entity::find_by_id(id)
        .one(state.db())
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
        state::mock::MockState,
        utils::testing::{assert_error, body_to_json},
    };
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        routing::{Router, get},
    };
    use chrono::Utc;
    use sea_orm::{DatabaseBackend, MockDatabase};
    use serde_json::json;
    use tower::ServiceExt;

    #[tokio::test]
    async fn has_in_db() {
        let state = MockState::builder()
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

        let app = Router::new()
            .route("/{id}", get(get_action::<MockState>))
            .with_state(state);

        let res = app
            .oneshot(Request::get("/TestID-0").body(Body::empty()).unwrap())
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
        let state = MockState::builder()
            .db(MockDatabase::new(DatabaseBackend::MySql)
                .append_query_results::<actions::Model, _, _>(vec![vec![]]))
            .build();

        let app = Router::new()
            .route("/{id}", get(get_action::<MockState>))
            .with_state(state);

        let res = app
            .oneshot(Request::get("/TestID-0").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_error!(res, error::ACTION_NOT_FOUND);
    }
}
