use async_graphql::{Context};
use rbatis::rbatis::Rbatis;
use crate::users::{self, models::User, models::MyLog};
use crate::util::constant::GqlResult;


pub struct Query;

#[async_graphql::Object]
impl Query {
    #[graphql(complexity = 5)]
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    async fn all_users(&self, ctx: &Context<'_> ) -> GqlResult<Vec<User>> {
        let my_pool = ctx.data_unchecked::<Rbatis>();
        users::services::all_users(my_pool).await
    }

    async fn all_logs(&self, ctx: &Context<'_> ) -> GqlResult<Vec<MyLog>> {
        let my_pool = ctx.data_unchecked::<Rbatis>();
        users::services::all_logs(my_pool).await
    }

    async fn get_user_by_email(&self, ctx: &Context<'_>, email: String) -> GqlResult<User> {
        let my_pool = ctx.data_unchecked::<Rbatis>();
        users::services::get_user_by_email(my_pool, &email).await
    }
}