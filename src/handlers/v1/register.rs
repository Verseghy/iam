use crate::password;
use actix_web::{http::StatusCode, route, web, Error, HttpResponse};
use entity::users;
use sea_orm::{
    entity::{ActiveValue::Set, EntityTrait},
    DatabaseConnection, DbErr,
};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct RegisterRequest {
    #[validate(email, length(max = 256))]
    email: String,
    #[validate(length(max = 256))]
    name: String,
    #[validate(length(max = 256))]
    password: String,
}

#[route("/v1/register", method = "POST")]
pub async fn register(
    req: web::Json<RegisterRequest>,
    db: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, Error> {
    let req = req.into_inner();

    if req.validate().is_err() {
        return Ok(HttpResponse::new(StatusCode::BAD_REQUEST));
    }

    if let Ok(hash) = password::encrypt(&req.password) {
        let action = users::ActiveModel {
            email: Set(req.email),
            name: Set(req.name),
            password: Set(hash),
            ..Default::default()
        };

        match users::Entity::insert(action).exec(db.get_ref()).await {
            Err(DbErr::Exec(_)) => Ok(HttpResponse::BadRequest().body("Email already exists")),
            Err(_) => Ok(HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)),
            _ => Ok(HttpResponse::new(StatusCode::OK)),
        }
    } else {
        Ok(HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR))
    }
}
