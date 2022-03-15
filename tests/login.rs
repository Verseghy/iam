mod utils;

use actix_web::{
    http::StatusCode,
    test::{call_service, TestRequest},
};
use sea_orm::{entity::EntityTrait, ActiveValue::*};
use serde_json::json;

#[actix_web::test]
async fn login() {
    let (mut app, _db) = utils::get_app().await;

    let req = TestRequest::post()
        .uri("/v1/register")
        .set_json(json!({
            "email": "test@test.test",
            "name": "test",
            "password": "test",
        }))
        .to_request();

    assert_eq!(call_service(&mut app, req).await.status(), StatusCode::OK);

    let req = TestRequest::post()
        .uri("/v1/login")
        .set_json(json!({
            "email": "test@test.test",
            "password": "test",
        }))
        .to_request();

    assert_eq!(call_service(&mut app, req).await.status(), StatusCode::OK);
}

#[actix_web::test]
async fn login_without_account() {
    let (mut app, _db) = utils::get_app().await;

    let req = TestRequest::post()
        .uri("/v1/login")
        .set_json(json!({
            "email": "test@test.test",
            "password": "test",
        }))
        .to_request();

    assert_eq!(
        call_service(&mut app, req).await.status(),
        StatusCode::UNAUTHORIZED
    );
}

#[actix_web::test]
async fn login_bad_password() {
    let (mut app, _db) = utils::get_app().await;

    let req = TestRequest::post()
        .uri("/v1/register")
        .set_json(json!({
            "email": "test@test.test",
            "name": "test",
            "password": "test",
        }))
        .to_request();

    assert_eq!(call_service(&mut app, req).await.status(), StatusCode::OK);

    let req = TestRequest::post()
        .uri("/v1/login")
        .set_json(json!({
            "email": "test@test.test",
            "password": "bad password",
        }))
        .to_request();

    assert_eq!(
        call_service(&mut app, req).await.status(),
        StatusCode::UNAUTHORIZED
    );
}

#[actix_web::test]
async fn login_long_email() {
    let (mut app, _db) = utils::get_app().await;

    let req = TestRequest::post()
        .uri("/v1/login")
        .set_json(json!({
            "email": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "password": "test",
        }))
        .to_request();

    assert_eq!(
        call_service(&mut app, req).await.status(),
        StatusCode::BAD_REQUEST
    );
}

#[actix_web::test]
async fn login_long_password() {
    let (mut app, _db) = utils::get_app().await;

    let req = TestRequest::post()
        .uri("/v1/login")
        .set_json(json!({
            "email": "test@test.test",
            "password": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        }))
        .to_request();

    assert_eq!(
        call_service(&mut app, req).await.status(),
        StatusCode::BAD_REQUEST
    );
}

#[actix_web::test]
async fn login_bad_email() {
    let (mut app, _db) = utils::get_app().await;

    let req = TestRequest::post()
        .uri("/v1/login")
        .set_json(json!({
            "email": "testtesttest",
            "password": "test",
        }))
        .to_request();

    assert_eq!(
        call_service(&mut app, req).await.status(),
        StatusCode::BAD_REQUEST
    );
}

#[actix_web::test]
async fn login_rehash() {
    use entity::users::{ActiveModel, Entity};

    let (mut app, db) = utils::get_app().await;

    let user = ActiveModel {
        email: Set("test@test.test".into()),
        name: Set("test".into()),
        password: Set("$2y$10$I/chY5TvXIdS7r.hbvr9xemtwEozCffCyItAe8bhJSz9jYtHtpMoa".into()),
        ..Default::default()
    };

    let res = Entity::insert(user).exec(&db).await.unwrap();

    let req = TestRequest::post()
        .uri("/v1/login")
        .set_json(json!({
            "email": "test@test.test",
            "password": "test",
        }))
        .to_request();

    assert_eq!(call_service(&mut app, req).await.status(), StatusCode::OK,);

    let res = Entity::find_by_id(res.last_insert_id)
        .one(&db)
        .await
        .unwrap()
        .unwrap();

    assert!(res.password.starts_with("$argon2id$"));
}
