use crate::schema::post::loader;
use crate::schema::post::model as post_model;
use async_graphql::FieldError;
use async_graphql::FieldResult;
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, SimpleObject};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, SimpleObject, Serialize, Deserialize)]
#[sea_orm(table_name = "user")]
#[graphql(complex, name = "User")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub create_at: DateTime,
    pub update_at: DateTime,
}

#[ComplexObject]
impl Model {
    async fn posts(&self, context: &Context<'_>) -> FieldResult<Vec<post_model::Model>> {
        let loader = context.data_unchecked::<DataLoader<loader::PostLoader>>();

        match loader.load_one(self.id).await {
            Ok(Some(posts)) => Ok(posts),
            Ok(None) => Err(FieldError::new("Not Found")),
            Err(e) => Err(FieldError::new(format!("{}", e))),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "post_model::Entity")]
    Post,
}

impl Related<post_model::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Post.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
