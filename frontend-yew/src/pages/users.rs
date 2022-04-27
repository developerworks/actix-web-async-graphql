use yew::{html, Html, Component, Context};

pub struct Users;

pub enum Msg {
}

impl Component for Users {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        // match msg {
        //     Msg::AddOne => {
        //         self.value += 1;
        //         // the value has changed so we need to
        //         // re-render for it to appear on the page
        //         true
        //     }
        // }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>{ "User page" }</div>
        }
    }
}