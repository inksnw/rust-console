use yew::prelude::*;

use crate::helper::js;

pub struct TestComp {
    pub name: String,
}

pub enum Msg {
    TestClick
}

impl Component for TestComp {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            name: String::from("haha")
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::TestClick => {
                js::alert("按钮点击");
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
            <button onclick={ctx.link().callback(|_| Msg::TestClick)}>{"点我"}  </button>
            <h1>{ &self.name }</h1>
            </div>
            }
    }
}