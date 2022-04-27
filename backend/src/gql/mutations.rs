use async_graphql::Context;
use rbatis::rbatis::Rbatis;

use crate::users::services;
use crate::users::models::{User, NewUser};

use crate::util::constant::GqlResult;

pub struct MutationRoot;

#[async_graphql::Object]
impl MutationRoot {
    // 插入新用户
    async fn new_user(
        &self,
        ctx: &Context<'_>,
        new_user: NewUser,
    ) -> GqlResult<User> {
        let my_pool = ctx.data_unchecked::<Rbatis>();
        services::create_user(my_pool, new_user).await
    }
}