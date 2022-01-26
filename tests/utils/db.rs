use iam::entity::actions::Entity;
use sea_orm::{ConnectionTrait, Database, DatabaseBackend, DatabaseConnection, Schema};

struct DB {
    db: Option<DatabaseConnection>,
}

impl DB {
    pub const fn new() -> Self {
        Self { db: None }
    }

    pub async fn get(&mut self) -> DatabaseConnection {
        if let Some(ref db) = self.db {
            db.clone()
        } else {
            let db = Database::connect("sqlite::memory:").await.unwrap();
            self.db = Some(db.clone());
            setup_schema(&db).await;
            db
        }
    }
}

static mut DATABASE: DB = DB::new();

pub async fn get_db() -> DatabaseConnection {
    unsafe { DATABASE.get().await }
}

async fn setup_schema(db: &DatabaseConnection) {
    let schema = Schema::new(DatabaseBackend::Sqlite);
    let stmt = schema.create_table_from_entity(Entity);

    db.execute(db.get_database_backend().build(&stmt))
        .await
        .unwrap();
}
