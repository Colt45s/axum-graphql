pub(crate) mod post;
mod user;
use async_graphql::EmptySubscription;
use async_graphql::MergedObject;

use self::post::resolver::PostMutation;
use self::user::resolver::UserQuery;

#[derive(MergedObject, Default)]
pub struct Query(UserQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(PostMutation);

pub type Schema = async_graphql::Schema<Query, Mutation, EmptySubscription>;
