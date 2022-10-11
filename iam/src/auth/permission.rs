use entity::{
    actions::{self, Entity as Actions},
    apps::{self, Entity as Apps},
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
    let actions: Vec<String> = ActionResult::find_by_statement(StatementBuilder::build(
        <Users as Related<Actions>>::find_related()
            .filter(users::Column::Id.eq(user_id))
            .select_only()
            .column(actions::Column::Name)
            .into_query()
            .union(
                UnionType::Distinct,
                Actions::get_actions_for_user_id(user_id)
                    .select_only()
                    .column(actions::Column::Name)
                    .into_query(),
            )
            .union(
                UnionType::Distinct,
                <Apps as Related<Actions>>::find_related()
                    .filter(apps::Column::Id.eq(user_id))
                    .select_only()
                    .column(actions::Column::Name)
                    .into_query(),
            )
            .union(
                UnionType::Distinct,
                Apps::get_actions_through_groups(user_id)
                    .select_only()
                    .column(actions::Column::Name)
                    .into_query(),
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
