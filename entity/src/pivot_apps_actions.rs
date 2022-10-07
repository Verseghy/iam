use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "pivot_actions_groups")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub app_id: String,
    #[sea_orm(primary_key)]
    pub action_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    App,
    Action,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::App => Entity::belongs_to(super::apps::Entity)
                .from(Column::AppId)
                .to(super::apps::Column::Id)
                .into(),
            Self::Action => Entity::belongs_to(super::actions::Entity)
                .from(Column::ActionId)
                .to(super::actions::Column::Id)
                .into(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}
