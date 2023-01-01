use common::{
    database,
    token::{Claims, Jwt, JwtTrait},
};
use entity::users;
use sea_orm::EntityTrait;
use std::env;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db = database::connect().await;
    let input = env::args().nth(1).expect("no input user");

    let jwt = Jwt::new();

    let user = users::Entity::find_by_id(input)
        .one(&db)
        .await
        .expect("database failed")
        .expect("no such user");

    let claims = Claims {
        subject: user.id,
        ..Default::default()
    };

    let token = jwt.encode(&claims).expect("failed to encode claims");
    println!("{token}");
}
