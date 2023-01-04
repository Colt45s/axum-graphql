pub(crate) mod post;
mod user;
use async_graphql::MergedObject;
use async_graphql::{EmptyMutation, EmptySubscription};

use self::user::resolver::UserQuery;

#[derive(MergedObject, Default)]
pub struct Query(UserQuery);

pub type Schema = async_graphql::Schema<Query, EmptyMutation, EmptySubscription>;
