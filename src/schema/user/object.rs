use async_graphql::dataloader::DataLoader;
use async_graphql::{ComplexObject, Context, FieldError, FieldResult, SimpleObject};
use itertools::Itertools;

use crate::entities::user::Model;
use crate::schema::post::loader;
use crate::schema::post::object::Post;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[ComplexObject]
impl User {
    async fn posts(&self, context: &Context<'_>) -> FieldResult<Vec<Post>> {
        let loader = context.data_unchecked::<DataLoader<loader::PostLoader>>();

        match loader.load_one(self.id).await {
            Ok(Some(posts)) => Ok(posts.into_iter().map(|post| post.into()).collect_vec()),
            Ok(None) => Err(FieldError::new("Not Found")),
            Err(e) => Err(FieldError::new(format!("{}", e))),
        }
    }
}

impl From<Model> for User {
    fn from(value: Model) -> Self {
        User {
            id: value.id,
            name: value.name,
        }
    }
}
