use common::{create_group_id, create_user_id, database, password};
use entity::{
    actions, groups, pivot_actions_groups, pivot_actions_users, pivot_users_groups, users,
};
use sea_orm::{ActiveModelTrait, ActiveValue::*, DbConn};

async fn create_user(db: &DbConn, name: &str, email: &str, password: &str) -> String {
    println!("create user: {}", name);
    users::ActiveModel {
        id: Set(create_user_id()),
        name: Set(name.to_string()),
        email: Set(email.to_string()),
        password: Set(password::hash(password).unwrap()),
        ..Default::default()
    }
    .insert(db)
    .await
    .unwrap()
    .id
}

async fn get_action(db: &DbConn, name: &str) -> String {
    println!("get action: {}", name);
    actions::Entity::find_by_name(name)
        .one(db)
        .await
        .unwrap()
        .unwrap()
        .id
}

async fn create_group(db: &DbConn, name: &str) -> String {
    println!("create group: {}", name);
    groups::ActiveModel {
        id: Set(create_group_id()),
        name: Set(name.to_string()),
        ..Default::default()
    }
    .insert(db)
    .await
    .unwrap()
    .id
}

async fn add_action_to_user(db: &DbConn, action: &str, user: &String) {
    println!("add action `{}` to user: {}", action, user);
    let action = get_action(db, action).await;

    pivot_actions_users::ActiveModel {
        action_id: Set(action),
        user_id: Set(user.clone()),
    }
    .insert(db)
    .await
    .unwrap();
}

async fn add_action_to_group(db: &DbConn, action: &str, group: &String) {
    println!("add action `{}` to group: {}", action, group);
    let action = get_action(db, action).await;

    pivot_actions_groups::ActiveModel {
        action_id: Set(action),
        group_id: Set(group.clone()),
    }
    .insert(db)
    .await
    .unwrap();
}

async fn add_user_to_group(db: &DbConn, user: &String, group: &String) {
    println!("add user `{}` to group: {}", user, group);
    pivot_users_groups::ActiveModel {
        user_id: Set(user.clone()),
        group_id: Set(group.clone()),
    }
    .insert(db)
    .await
    .unwrap();
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let db = database::connect().await;

    let user1 = create_user(&db, "TestUser1", "test@test.test", "test").await;
    let _user2 = create_user(&db, "TestUser2", "test2@test.test", "test").await;

    let admin_group = create_group(&db, "admin").await;

    add_user_to_group(&db, &user1, &admin_group).await;

    add_action_to_group(&db, "iam.action.add", &admin_group).await;
    add_action_to_group(&db, "iam.action.update", &admin_group).await;
    add_action_to_group(&db, "iam.action.delete", &admin_group).await;

    add_action_to_user(&db, "iam.action.get", &user1).await;
    add_action_to_user(&db, "iam.action.list", &user1).await;
}
