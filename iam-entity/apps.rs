use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{Set, entity::prelude::*};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "apps")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub name: String,
    pub password: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl Related<super::actions::Entity> for Entity {
    fn to() -> RelationDef {
        super::pivot_apps_actions::Relation::Action.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::pivot_apps_actions::Relation::App.def().rev())
    }
}

impl Related<super::groups::Entity> for Entity {
    fn to() -> RelationDef {
        super::pivot_apps_groups::Relation::Group.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::pivot_apps_groups::Relation::App.def())
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
