use yew::prelude::*;

pub struct Home;

pub enum Msg {
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>{ "Home page" }</div>
        }
    }
}