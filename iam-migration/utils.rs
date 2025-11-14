use iam_common::Id;
use iam_entity::{actions, pivot_actions_groups, pivot_actions_users, pivot_apps_actions};
use sea_orm::{ConnectionTrait, Set, prelude::*};
use sea_orm_migration::prelude::*;

pub async fn add_action(
    txn: &impl ConnectionTrait,
    action: &str,
    secure: bool,
) -> Result<(), DbErr> {
    let model = actions::ActiveModel {
        id: Set(Id::new_action().to_string()),
        name: Set(action.to_owned()),
        secure: Set(secure),
        ..Default::default()
    };

    actions::Entity::insert(model).exec(txn).await?;

    Ok(())
}

pub async fn delete_action(txn: &impl ConnectionTrait, name: &str) -> Result<(), DbErr> {
    let action_id = actions::Entity::find_by_name(name)
        .one(txn)
        .await?
        .expect("no such action")
        .id;

    actions::Entity::delete_by_id(action_id.clone())
        .exec(txn)
        .await?;

    pivot_actions_groups::Entity::delete_many()
        .filter(pivot_actions_groups::Column::ActionId.eq(action_id.clone()))
        .exec(txn)
        .await?;

    pivot_actions_users::Entity::delete_many()
        .filter(pivot_actions_users::Column::ActionId.eq(action_id.clone()))
        .exec(txn)
        .await?;

    pivot_apps_actions::Entity::delete_many()
        .filter(pivot_apps_actions::Column::ActionId.eq(action_id.clone()))
        .exec(txn)
        .await?;

    Ok(())
}
