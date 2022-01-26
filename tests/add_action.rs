mod utils;

use actix_web::{
    http::StatusCode,
    test::{call_service, TestRequest},
};
use iam::entity::actions::{Entity as Actions, Model};
use sea_orm::EntityTrait;
use serde_json::json;
use serial_test::serial;

#[actix_web::test]
#[serial]
async fn test_add_action() {
    let db = utils::get_db().await;
    let mut app = utils::get_service().await;

    let req = TestRequest::post()
        .append_header(("Authorization", "valami token"))
        .uri("/v1/action")
        .set_json(json!({
            "action": "post.create",
            "secure": true,
        }))
        .to_request();

    let res = call_service(&mut app, req).await;

    assert_eq!(res.status(), StatusCode::OK);

    let res = Actions::find().all(&db).await;

    assert_eq!(
        res,
        Ok(vec![Model {
            id: 1,
            name: "post.create".to_string(),
            secure: 1,
        }])
    )
}
