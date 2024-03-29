//! SeaORM Entity. Generated by sea-orm-codegen 0.7.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "pivot_actions_groups")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub action_id: String,
    #[sea_orm(primary_key)]
    pub group_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Action,
    Group,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Action => Entity::belongs_to(super::actions::Entity)
                .from(Column::ActionId)
                .to(super::actions::Column::Id)
                .into(),
            Self::Group => Entity::belongs_to(super::groups::Entity)
                .from(Column::GroupId)
                .to(super::groups::Column::Id)
                .into(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}
