use super::model::Entity as UserEntity;
use super::model::Model as UserModel;
use async_graphql::{Context, FieldError, FieldResult, Object};
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;

#[derive(Default)]
pub struct UserQuery {}

#[Object]
impl UserQuery {
    pub async fn users(&self, context: &Context<'_>) -> FieldResult<Vec<UserModel>> {
        let conn = context.data::<DatabaseConnection>().unwrap();

        match UserEntity::find().all(conn).await {
            Ok(users) => Ok(users),
            Err(e) => Err(FieldError::new(format!("{}", e))),
        }
    }
    pub async fn user(&self, context: &Context<'_>, user_id: i32) -> FieldResult<UserModel> {
        let conn = context.data::<DatabaseConnection>().unwrap();

        match UserEntity::find_by_id(user_id).one(conn).await {
            Ok(Some(user)) => Ok(user),
            Ok(None) => Err(FieldError::new("Not Found")),
            Err(e) => Err(FieldError::new(format!("{}", e))),
        }
    }
}
