use yew::prelude::*;

pub struct NotFound;

pub enum Msg {
}

impl Component for NotFound {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>{"Not Found"}</div>
        }
    }
}