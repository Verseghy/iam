use entity::users;
use iam::{database, id::create_id, password};
use sea_orm::{ActiveModelTrait, ActiveValue::*};

#[actix_web::main]
async fn main() {
    dotenv::dotenv().ok();
    let db = database::connect().await;

    users::ActiveModel {
        id: Set(create_id()),
        name: Set("TestUser1".into()),
        email: Set("test@test.test".into()),
        password: Set(password::encrypt("test").unwrap()),
        ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();

    users::ActiveModel {
        id: Set(create_id()),
        name: Set("TestUser2".into()),
        email: Set("test2@test.test".into()),
        password: Set(password::encrypt("test").unwrap()),
        ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();
}
