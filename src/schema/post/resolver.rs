use super::model::post::post_model;
use async_graphql::*;

#[derive(InputObject)]
struct CreatePostInput {
    user_id: i32,
}

struct Mutation;

#[Object]
impl Mutation {
    async fn create_post(&self, input: CreatePostInput) -> post_model::Model {
        // Writes coordination to database.
        // ...
    }
}
