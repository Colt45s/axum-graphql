use crate::entities::post::Model;
use async_graphql::SimpleObject;
use sea_orm::entity::prelude::DateTime;

#[derive(SimpleObject)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub create_at: DateTime,
    pub update_at: DateTime,
}

impl From<Model> for Post {
    fn from(value: Model) -> Self {
        Post {
            id: value.id,
            title: value.title,
            create_at: value.create_at,
            update_at: value.update_at,
        }
    }
}
