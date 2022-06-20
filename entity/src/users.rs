use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub name: String,
    #[sea_orm(unique)]
    pub email: String,
    pub password: String,
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
        super::pivot_actions_users::Relation::Action.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::pivot_actions_users::Relation::User.def().rev())
    }
}

impl Related<super::groups::Entity> for Entity {
    fn to() -> RelationDef {
        super::pivot_users_groups::Relation::Group.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::pivot_users_groups::Relation::User.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
