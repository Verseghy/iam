mod utils;

use actix_web::{
    http::StatusCode,
    test::{call_service, TestRequest},
};
use entity::users;
use sea_orm::EntityTrait;
use serde_json::json;

#[actix_web::test]
async fn register() {
    let (mut app, db) = utils::get_app().await;

    let req = TestRequest::post()
        .uri("/v1/register")
        .set_json(json!({
            "email": "test@test.test",
            "name": "valami",
            "password": "asd",
        }))
        .to_request();

    assert_eq!(call_service(&mut app, req).await.status(), StatusCode::OK);

    let res = users::Entity::find().all(&db).await.unwrap();

    assert_eq!(res.len(), 1);
}

#[actix_web::test]
async fn register_bad_email() {
    let (mut app, _db) = utils::get_app().await;

    let req = TestRequest::post()
        .uri("/v1/register")
        .set_json(json!({
            "email": "test.test",
            "name": "valami",
            "password": "asd",
        }))
        .to_request();

    assert_eq!(
        call_service(&mut app, req).await.status(),
        StatusCode::BAD_REQUEST
    );
}

#[actix_web::test]
async fn register_long_email() {
    let (mut app, _db) = utils::get_app().await;

    let req = TestRequest::post()
        .uri("/v1/register")
        .set_json(json!({
            "email": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa@a.a",
            "name": "valami",
            "password": "asd",
        }))
        .to_request();

    assert_eq!(
        call_service(&mut app, req).await.status(),
        StatusCode::BAD_REQUEST
    );
}

#[actix_web::test]
async fn register_long_name() {
    let (mut app, _db) = utils::get_app().await;

    let req = TestRequest::post()
        .uri("/v1/register")
        .set_json(json!({
            "email": "test@test.test",
            "name": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "password": "asd",
        }))
        .to_request();

    assert_eq!(
        call_service(&mut app, req).await.status(),
        StatusCode::BAD_REQUEST
    );
}

#[actix_web::test]
async fn register_long_password() {
    let (mut app, _db) = utils::get_app().await;

    let req = TestRequest::post()
        .uri("/v1/register")
        .set_json(json!({
            "email": "test@test.test",
            "name": "test",
            "password": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        }))
        .to_request();

    assert_eq!(
        call_service(&mut app, req).await.status(),
        StatusCode::BAD_REQUEST
    );
}
