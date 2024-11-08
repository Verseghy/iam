use async_trait::async_trait;
use chrono::Utc;
use sea_orm::entity::prelude::*;
use sea_orm::{JoinType, QuerySelect, Set};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "actions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(unique)]
    pub name: String,
    pub secure: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        super::pivot_actions_users::Relation::User.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::pivot_actions_users::Relation::Action.def().rev())
    }
}

impl Related<super::groups::Entity> for Entity {
    fn to() -> RelationDef {
        super::pivot_actions_groups::Relation::Group.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::pivot_actions_groups::Relation::Action.def().rev())
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        self.updated_at = Set(Utc::now().naive_utc());
        Ok(self)
    }
}

impl Entity {
    pub fn get_actions_for_user_id_through_groups(id: &str) -> Select<super::actions::Entity> {
        Self::find()
            .filter(super::users::Column::Id.eq(id))
            .join_rev(
                JoinType::InnerJoin,
                super::pivot_actions_groups::Relation::Action.def(),
            )
            .join(
                JoinType::InnerJoin,
                super::pivot_actions_groups::Relation::Group.def(),
            )
            .join_rev(
                JoinType::InnerJoin,
                super::pivot_users_groups::Relation::Group.def(),
            )
            .join(
                JoinType::InnerJoin,
                super::pivot_users_groups::Relation::User.def(),
            )
    }

    pub fn get_actions_for_app_id_through_groups(id: &str) -> Select<super::actions::Entity> {
        super::actions::Entity::find()
            .filter(Column::Id.eq(id))
            .join_rev(
                JoinType::InnerJoin,
                super::pivot_actions_groups::Relation::Action.def(),
            )
            .join(
                JoinType::InnerJoin,
                super::pivot_actions_groups::Relation::Group.def(),
            )
            .join_rev(
                JoinType::InnerJoin,
                super::pivot_apps_groups::Relation::Group.def(),
            )
            .join(
                JoinType::InnerJoin,
                super::pivot_apps_groups::Relation::App.def(),
            )
    }

    pub fn find_by_name(name: &str) -> Select<Entity> {
        Self::find().filter(Column::Name.eq(name))
    }
}
