use actix_web::{http::StatusCode, route, web, Error, HttpResponse};
use entity::actions;
use sea_orm::{
    entity::{
        ActiveValue::{NotSet, Set},
        EntityTrait,
    },
    DatabaseConnection,
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AddActionRequest {
    action: String,
    secure: bool,
}

#[route("/v1/action", method = "POST")]
pub async fn add_action(
    req: web::Json<AddActionRequest>,
    db: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, Error> {
    let req = req.into_inner();

    let action = actions::ActiveModel {
        id: NotSet,
        name: Set(req.action),
        secure: Set(req.secure),
        ..Default::default()
    };

    actions::Entity::insert(action)
        .exec(db.get_ref())
        .await
        .unwrap();

    Ok(HttpResponse::new(StatusCode::OK))
}
