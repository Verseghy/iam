//! SeaORM Entity. Generated by sea-orm-codegen 0.7.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "pivot_users_groups")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id: String,
    #[sea_orm(primary_key)]
    pub group_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    User,
    Group,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::User => Entity::belongs_to(super::users::Entity)
                .from(Column::UserId)
                .to(super::users::Column::Id)
                .into(),
            Self::Group => Entity::belongs_to(super::groups::Entity)
                .from(Column::GroupId)
                .to(super::groups::Column::Id)
                .into(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}
