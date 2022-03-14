mod utils;

use actix_web::{
    http::StatusCode,
    test::{call_service, TestRequest},
};
use entity::actions::{Entity as Actions};
use sea_orm::EntityTrait;
use serde_json::json;

#[actix_web::test]
async fn test_add_action() {
    let (mut app, db) = utils::get_app().await;

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

    let _res = Actions::find().all(&db).await;

    // assert_eq!(
    //     res,
    //     Ok(vec![Model {
    //         id: 1,
    //         name: "post.create".to_string(),
    //         secure: true,
    //         created_at: Local::now().naive_local(),
    //         updated_at: Local::now().naive_local(),
    //         deleted_at: None,
    //     }])
    // )
}
