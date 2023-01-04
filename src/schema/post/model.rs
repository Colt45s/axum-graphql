use crate::schema::user::model::{Column as UserColumn, Entity as UserEntity};
use async_graphql::SimpleObject;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, SimpleObject, Serialize, Deserialize)]
#[sea_orm(table_name = "post")]
#[graphql(name = "Post")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    pub create_at: DateTime,
    pub update_at: DateTime,
    pub user_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "UserEntity",
        from = "Column::UserId",
        to = "UserColumn::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User,
}

impl Related<UserEntity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
