use sea_orm::{entity::prelude::*, JoinType, QueryFilter, QuerySelect};

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

impl ActiveModelBehavior for ActiveModel {}

impl Related<super::actions::Entity> for Entity {
    fn to() -> RelationDef {
        super::pivot_apps_actions::Relation::Action.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::pivot_apps_actions::Relation::App.def())
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

impl Entity {
    pub fn get_actions_through_groups(id: &str) -> Select<super::actions::Entity> {
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
}
