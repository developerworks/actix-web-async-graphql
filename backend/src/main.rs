mod gql;
mod dbs;
mod users;
mod util;

use actix_web::{http::header, guard, web, App, HttpServer};
use actix_web::middleware::{Logger};
use crate::gql::{build_schema, graphql, graphiql};
use actix_cors::Cors;
use env_logger;
use crate::util::constant::CFG;
// use log::debug;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting graphql server at: http://{}:{}", CFG.get("ADDRESS").unwrap(), CFG.get("PORT").unwrap());
    // 日志等级
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "actix_web=debug");
    }

    // 日志初始化
    env_logger::init();

    let schema = build_schema().await;

    HttpServer::new(move || {
        App::new()
            // 应用数据
            .app_data(web::Data::new(schema.clone()))

            // 日志, Actix 为什么默认没有日志, 难道是作为一种常见的部署策略,在 Actix 的前端要设置反向代理服务器?
            // 但是如果没有设置日志框架, 那么错误信息怎么获取, 怎么排错?
            .wrap(Logger::default())

            // 跨域 CORS
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["DELETE", "OPTIONS", "POST", "PUT", "HEAD", "CONNECT", "TRACE", "GET", "PATCH"])
                    .allowed_headers(vec![header::CONTENT_TYPE, header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(60 * 60 * 24),
            )
            .service(web::resource("/graphql").guard(guard::Post()).to(graphql))
            .service(web::resource("/graphiql").guard(guard::Get()).to(graphiql))
    }).bind(format!(
        "{}:{}",
        CFG.get("ADDRESS").unwrap(),
        CFG.get("PORT").unwrap()
    ))?.run().await
}
