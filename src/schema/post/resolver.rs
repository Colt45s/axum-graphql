use crate::entities::post;
use async_graphql::*;
use chrono::Local;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

use super::object::Post;

#[derive(InputObject)]
struct CreatePostInput {
    user_id: i32,
    title: String,
}

#[derive(Default)]
pub struct PostMutation {}

#[Object]
impl PostMutation {
    async fn create_post(&self, context: &Context<'_>, input: CreatePostInput) -> Result<Post> {
        let conn = context.data::<DatabaseConnection>().unwrap();

        let post = post::ActiveModel {
            title: Set(input.title),
            create_at: Set(Local::now().naive_local()),
            update_at: Set(Local::now().naive_local()),
            user_id: Set(input.user_id),
            ..Default::default()
        };

        match post.insert(conn).await {
            Ok(post) => Ok(post.into()),
            Err(e) => Err(FieldError::new(format!("{}", e))),
        }
    }
}
