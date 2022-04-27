use tide::{self, Server, Request};
use serde_json::json;

use crate::{State, util::common::Tpl};

pub async fn push_res(app: &mut Server<State>) {

    app.at("/").get(index);
}

async fn index(_req: Request<State>) -> tide::Result {
    let index: Tpl = Tpl::new("index").await;

    // make data and render it
    let data = json!({"app_name": "frontend-handlebars - tide-async-graphql-mongodb", "author": "zzy"});

    index.render(&data).await
}