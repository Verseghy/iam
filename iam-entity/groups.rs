use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, Set};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "groups")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(unique)]
    pub name: String,
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

impl Related<super::actions::Entity> for Entity {
    fn to() -> RelationDef {
        super::pivot_actions_groups::Relation::Action.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::pivot_actions_groups::Relation::Group.def().rev())
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        super::pivot_users_groups::Relation::User.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::pivot_users_groups::Relation::Group.def().rev())
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
