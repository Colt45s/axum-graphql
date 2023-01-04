use async_graphql::async_trait;
use async_graphql::dataloader::*;
use itertools::Itertools;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::{DatabaseConnection, DbErr};
use std::{collections::HashMap, sync::Arc};

use super::model::{Column as PostColumn, Entity as PostEntity, Model as PostModel};

pub(crate) struct PostLoader {
    conn: DatabaseConnection,
}

impl PostLoader {
    pub fn new(conn: DatabaseConnection) -> Self {
        PostLoader { conn }
    }
}

#[async_trait::async_trait]
impl Loader<i32> for PostLoader {
    type Value = Vec<PostModel>;
    type Error = Arc<DbErr>;

    async fn load(&self, keys: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        let target_column: PostColumn = PostColumn::UserId;
        let posts = PostEntity::find()
            .filter(target_column.is_in(keys.to_vec()))
            .all(&self.conn)
            .await?;

        Ok(posts.into_iter().into_group_map_by(|post| post.user_id))
    }
}
