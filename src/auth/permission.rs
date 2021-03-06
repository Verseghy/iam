use entity::{
    actions::{self, Entity as Actions},
    users::{self, Entity as Users},
};
use sea_orm::{
    error::DbErr,
    query::{QueryFilter, QuerySelect},
    sea_query::UnionType,
    ColumnTrait, ConnectionTrait, FromQueryResult, QueryTrait, Related, StatementBuilder,
};

#[derive(FromQueryResult)]
struct ActionResult {
    name: String,
}

pub async fn check<DB>(user_id: &str, permissions: &[&str], database: &DB) -> Result<(), CheckError>
where
    DB: ConnectionTrait,
{
    let mut user_permissions = <Users as Related<Actions>>::find_related()
        .filter(users::Column::Id.eq(user_id))
        .select_only()
        .column(actions::Column::Name);

    let actions: Vec<String> = ActionResult::find_by_statement(StatementBuilder::build(
        QueryTrait::query(&mut user_permissions).union(
            UnionType::Distinct,
            QueryTrait::into_query(
                Actions::get_actions_for_user_id(user_id)
                    .select_only()
                    .column(actions::Column::Name),
            ),
        ),
        &database.get_database_backend(),
    ))
    .all(database)
    .await?
    .into_iter()
    .map(|x| x.name)
    .collect();

    for permission in permissions {
        let mut has = false;
        for action in &actions {
            if action == permission {
                has = true;
                break;
            }
        }

        if !has {
            Err(CheckError::NoPermission(permission.to_string()))?;
        }
    }

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum CheckError {
    #[error("database error: {0}")]
    DatabaseError(#[from] DbErr),
    #[error("no permission: {0}")]
    NoPermission(String),
}
