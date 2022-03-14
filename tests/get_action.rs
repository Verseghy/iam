mod utils;

use actix_web::{
    http::StatusCode,
    test::{call_service, TestRequest},
};
use serde_json::json;

#[actix_web::test]
async fn test_get_action() {
    let (mut app, _db) = utils::get_app().await;

    let req = TestRequest::get()
        .uri("/v1/action/post.create")
        .to_request();

    assert_eq!(
        call_service(&mut app, req).await.status(),
        StatusCode::NOT_FOUND
    );

    let req = TestRequest::post()
        .append_header(("Authorization", "valami token"))
        .uri("/v1/action")
        .set_json(json!({
            "action": "post.create",
            "secure": true,
        }))
        .to_request();

    assert_eq!(call_service(&mut app, req).await.status(), StatusCode::OK);

    let req = TestRequest::get()
        .uri("/v1/action/post.create")
        .to_request();

    assert_eq!(call_service(&mut app, req).await.status(), StatusCode::OK);
}
