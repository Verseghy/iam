use entity::actions;
use actix_web::{http::StatusCode, route, web, Error, HttpResponse};
use sea_orm::{entity::EntityTrait, query::QueryFilter, ColumnTrait, DatabaseConnection};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GetActionRequest {
    action: String,
}

#[route("/v1/action/{action}", method = "GET")]
pub async fn get_action(
    req: web::Path<GetActionRequest>,
    db: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, Error> {
    let req = req.into_inner();

    let res = actions::Entity::find()
        .filter(actions::Column::Name.eq(req.action))
        .one(db.get_ref())
        .await
        .unwrap();

    if res.is_some() {
        Ok(HttpResponse::new(StatusCode::OK))
    } else {
        Ok(HttpResponse::new(StatusCode::NOT_FOUND))
    }
}
