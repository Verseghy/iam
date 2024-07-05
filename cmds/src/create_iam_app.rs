use std::env::{self, args};

use dotenvy::dotenv;
use libiam::testing::{apps::create_app, Database};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let name = args().nth(1).expect("no name given");

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let database = Database::connect(&db_url).await;

    let (id, secret) = create_app(&database, &name).await;

    println!("id: {}", id);
    println!("secret: {}", secret);
}
