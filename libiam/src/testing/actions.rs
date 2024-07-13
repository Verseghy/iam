use iam_common::Id;
use iam_entity::{actions, pivot_actions_users, pivot_apps_actions};
use sea_orm::{ConnectionTrait, EntityTrait, Set};

async fn get_action_by_name<D>(db: &D, name: &str) -> Option<String>
where
    D: ConnectionTrait,
{
    let res = actions::Entity::find_by_name(name).one(db).await.unwrap();

    res.map(|a| a.id)
}

async fn create_action<D>(db: &D, action_name: &str, secure: bool)
where
    D: ConnectionTrait,
{
    let id = Id::new_action();

    let model = actions::ActiveModel {
        id: Set(id.to_string()),
        name: Set(action_name.to_owned()),
        secure: Set(secure),
        ..Default::default()
    };

    let _ = actions::Entity::insert(model).exec(db).await;
}

pub async fn ensure_action<D>(db: &D, action_name: &str, secure: bool)
where
    D: ConnectionTrait,
{
    if get_action_by_name(db, action_name).await.is_none() {
        create_action(db, action_name, secure).await;
    }
}

pub async fn assign_action_to_user<D>(db: &D, action_name: &str, user_id: &str)
where
    D: ConnectionTrait,
{
    let action_id = get_action_by_name(db, action_name).await;

    if let Some(action_id) = action_id {
        let model = pivot_actions_users::ActiveModel {
            user_id: Set(user_id.to_owned()),
            action_id: Set(action_id),
        };

        pivot_actions_users::Entity::insert(model)
            .exec(db)
            .await
            .unwrap();
    }
}

pub async fn assign_action_to_app<D>(db: &D, action_name: &str, app_id: &str)
where
    D: ConnectionTrait,
{
    let action_id = get_action_by_name(db, action_name).await;

    if let Some(action_id) = action_id {
        tracing::debug!("assigning action '{action_id}' to app '{app_id}'");

        let model = pivot_apps_actions::ActiveModel {
            app_id: Set(app_id.to_string()),
            action_id: Set(action_id),
        };

        pivot_apps_actions::Entity::insert(model)
            .exec(db)
            .await
            .unwrap();
    }
}
