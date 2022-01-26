mod db;

pub use db::get_db;

use actix_http::Request;
use actix_service::Service;
use actix_web::{body::BoxBody, dev::ServiceResponse, test::init_service, web::Data, App};
use iam::routes;

pub async fn get_service(
) -> impl Service<Request, Response = ServiceResponse<BoxBody>, Error = actix_web::Error> {
    let db = get_db().await;
    init_service(App::new().app_data(Data::new(db)).configure(routes)).await
}
