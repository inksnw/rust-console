use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures;
use yew::prelude::*;

use crate::apis::test::{get_user_future, TestMsg, UserTrait};
use crate::helper::js;

pub struct TestComp {
    pub name: String,
}


#[derive(Clone, PartialEq, Deserialize, Serialize)]
struct User {
    message: String,
}

impl UserTrait for User {
    fn get_name(self) -> String {
        return self.message;
    }
}

impl Component for TestComp {
    type Message = TestMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(TestMsg::LoadUser);
        Self {
            name: String::from("haheeeeea")
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TestMsg::TestClick => {
                js::alert("按钮点击");
                true
            }
            TestMsg::LoadUser => {
                ctx.link().send_future(get_user_future::<User>());
                true
            }
            TestMsg::LoadUserDone(data) => {
                self.name = data;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
            <button onclick={ctx.link().callback(|_| TestMsg::TestClick)}>{"点我"}  </button>
            <h1>{ &self.name }</h1>
            </div>
            }
    }
}