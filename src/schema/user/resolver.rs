use super::object::User;
use crate::entities::user;
use async_graphql::{Context, FieldError, FieldResult, Object};
use itertools::Itertools;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;

#[derive(Default)]
pub struct UserQuery {}

#[Object]
impl UserQuery {
    pub async fn users(&self, context: &Context<'_>) -> FieldResult<Vec<User>> {
        let conn = context.data::<DatabaseConnection>().unwrap();

        match user::Entity::find().all(conn).await {
            Ok(users) => Ok(users.into_iter().map(|user| user.into()).collect_vec()),
            Err(e) => Err(FieldError::new(format!("{}", e))),
        }
    }

    pub async fn user(&self, context: &Context<'_>, user_id: i32) -> FieldResult<User> {
        let conn = context.data::<DatabaseConnection>().unwrap();

        match user::Entity::find_by_id(user_id).one(conn).await {
            Ok(Some(user)) => Ok(user.into()),
            Ok(None) => Err(FieldError::new("Not Found")),
            Err(e) => Err(FieldError::new(format!("{}", e))),
        }
    }
}
