use serde::{Serialize, Deserialize};
use async_graphql::*;
use rbatis::crud_table;


// 使用 SimpleObject 定义简单对象
// 使用 ComplexObject 定义包含计算字段的对象
#[crud_table]
#[derive(SimpleObject, InputObject, Serialize, Deserialize)]
#[graphql(complex, input_name = "UserInput")]
// #[graphql(input_name = "UserInput")]
pub struct User {
    pub id: i32,
    #[graphql(visible = "is_admin")]
    pub email: String,
    pub username: String,
    #[graphql(skip)]
    pub cred: String,
}

// struct IsAdmin(bool);
fn is_admin(_ctx: &Context<'_>) -> bool {
    // ctx.data_unchecked::<IsAdmin>().0
    true
}

#[crud_table]
#[derive(SimpleObject, Serialize, Deserialize)]
pub struct MyLog {
    pub id: i32,
    pub created_at: i32,
    pub msg: String,
}


#[async_graphql::ComplexObject]
impl User {
    pub async fn from(&self) -> String {
        let mut from = String::new();
        from.push_str(&self.username);
        from.push_str("<");
        from.push_str(&self.email);
        from.push_str(">");

        from
    }
}
#[rbatis::crud_table(table_name:"user")]
#[derive(async_graphql::InputObject, Serialize, Deserialize, Clone, Debug)]
pub struct NewUser {
    #[graphql(skip)]
    pub id: i32,
    pub email: String,
    pub username: String,
    #[graphql(skip)]
    pub cred: String,
}