use yew::prelude::*;
use yew_router::prelude::*;
mod pages;
use pages::{
    home::Home, 
    not_found::NotFound,
};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/secure")]
    Secure,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Secure)]
fn secure() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(Route::Home));
    html! {
        <div>
            <h1>{ "Secure" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}

// 组件和路由
fn switch(routes: &Route) -> Html {
    match routes.clone() {
        Route::Home => html! { <Home/> },
        Route::Secure => html! {
            <Secure />
        },
        Route::NotFound => html! { <NotFound/> },
    }
}

// 根组件
// Switch::render 函数通过其参数 switch 回调函数来判断渲染哪一个组件
#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}


// 程序入口
fn main() {
    yew::Renderer::<App>::new().render();
}