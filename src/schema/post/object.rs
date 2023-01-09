use crate::entities::post::Model;
use async_graphql::Object;
use sea_orm::entity::prelude::DateTime;

pub struct Post {
    pub model: Model,
}

#[Object(name = "Post")]
impl Post {
    async fn id(&self) -> i32 {
        self.model.id
    }

    async fn title(&self) -> String {
        self.model.title.to_string()
    }

    async fn create_at(&self) -> DateTime {
        self.model.create_at
    }

    async fn update_at(&self) -> DateTime {
        self.model.update_at
    }
}
