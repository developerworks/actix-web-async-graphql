use yew::prelude::*;

pub struct Projects;

pub enum Msg {
}

impl Component for Projects {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            
        }
    }
}