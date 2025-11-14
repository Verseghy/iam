use iam_common::error::{self, Result};
use iam_entity::{
    actions::{self, Entity as Actions},
    apps::{self, Entity as Apps},
    users::{self, Entity as Users},
};
use sea_orm::{
    ColumnTrait, ConnectionTrait, FromQueryResult, QueryTrait, Related, StatementBuilder,
    query::{QueryFilter, QuerySelect},
    sea_query::UnionType,
};

#[derive(FromQueryResult)]
struct ActionResult {
    name: String,
}

pub async fn check<DB>(user_id: &str, permissions: &[&str], database: &DB) -> Result<()>
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
                Actions::get_actions_for_user_id_through_groups(user_id)
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
                Actions::get_actions_for_app_id_through_groups(user_id)
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
            return Err(error::NO_PERMISSION);
        }
    }

    Ok(())
}
