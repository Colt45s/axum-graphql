use async_graphql::dataloader::DataLoader;
use async_graphql::{Context, FieldError, FieldResult, Object};
use itertools::Itertools;

use crate::entities::user::Model;
use crate::schema::post::loader;
use crate::schema::post::object::Post;
// use crate::schema::post::model as post_model;

pub struct User {
    pub model: Model,
}

#[Object(name = "User")]
impl User {
    async fn id(&self) -> i32 {
        self.model.id
    }

    async fn name(&self) -> String {
        self.model.name.to_string()
    }

    async fn posts(&self, context: &Context<'_>) -> FieldResult<Vec<Post>> {
        let loader = context.data_unchecked::<DataLoader<loader::PostLoader>>();

        match loader.load_one(self.model.id).await {
            Ok(Some(posts)) => Ok(posts
                .into_iter()
                .map(|post| Post { model: post })
                .collect_vec()),
            Ok(None) => Err(FieldError::new("Not Found")),
            Err(e) => Err(FieldError::new(format!("{}", e))),
        }
    }
}
