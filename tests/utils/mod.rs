use actix_http::Request;
use actix_service::Service;
use actix_web::{body::BoxBody, dev::ServiceResponse, test::init_service, web::Data, App};
use iam::connect;
use iam::routes;
use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;

async fn get_db() -> DatabaseConnection {
    dotenv::dotenv().ok();
    let conn = connect().await;
    Migrator::refresh(&conn)
        .await
        .expect("Failed to setup database!");
    conn
}

pub async fn get_app() -> (
    impl Service<Request, Response = ServiceResponse<BoxBody>, Error = actix_web::Error>,
    DatabaseConnection,
) {
    let db = get_db().await;
    let app = init_service(App::new().app_data(Data::new(db.clone())).configure(routes)).await;
    (app, db)
}
