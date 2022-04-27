pub mod mutations;
pub mod queries;


use actix_web::{web, HttpResponse, Result};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{ EmptySubscription, Schema};
// use async_graphql::extensions::ApolloTracing;

use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use crate::gql::queries::Query;
use crate::gql::mutations::MutationRoot;
use crate::dbs::mysql::connection_pool;

// `ActixSchema` 类型定义,项目中可以统一放置在一个共用文件中.
// 但和 `actix-web` 和 `tide` 框架不同,无需放入应用程序`状态（State）`
// 所以此 `Schema` 类型仅是为了代码清晰易读,使用位置并不多,我们直接和构建函数一起定义.
// 或者,不做此类型定义,直接作为构建函数的返回类型.

type ActixSchema = Schema<
    queries::Query,
    mutations::MutationRoot,
    async_graphql::EmptySubscription,
>;

pub async fn build_schema() -> ActixSchema {
    // 获取 MySql 数据池后,可以将其增加到：
    // 1. 作为 async-graphql 的全局数据；
    // 2. 作为 actix-web 的应用程序数据,优势是可以进行原子操作；
    // 3. 使用 lazy-static.rs
    let pool = connection_pool().await;
    // The root object for the query and Mutatio, and use EmptySubscription.
    // Add global sql datasource  in the schema object.

    Schema::build(Query, MutationRoot, EmptySubscription)
        // 启用ApolloTracing扩展
        // Apollo Tracing提供了查询每个步骤的性能分析结果,它是一个Schema扩展,性能分析结果保存在QueryResponse中.
        // 启用Apollo Tracing扩展需要在创建Schema的时候添加该扩展.
        // https://async-graphql.github.io/async-graphql/zh-CN/apollo_tracing.html
        // 需要在 Cargo.toml 中期启用 apollo_tracing
        // #async-graphql = { version = "3.0.38", features = ["apollo_tracing"] }
        // .extension(ApolloTracing)
        // https://async-graphql.github.io/async-graphql/zh-CN/depth_and_complexity.html
        // 限制最大深度为5
        // .limit_depth(20)
        // 限制最大复杂度为5
        // .limit_complexity(100)
        .data(pool)
        .finish()
}

pub async fn graphql(schema: web::Data<ActixSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

pub async fn graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(
        playground_source(
            GraphQLPlaygroundConfig::new("/graphql")
                .subscription_endpoint("/graphql"),
        ),
    ))
}