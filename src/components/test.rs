use gloo_net::http::Request;
use k8s_openapi::api::core::v1::Namespace;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures;
use yew::prelude::*;

use crate::apis::test::TestMsg;
use crate::helper::js;

use super::selectns::NameSpaceSelect;

pub struct TestComp {
    nslist: Vec<Namespace>,
}


#[derive(Clone, PartialEq, Deserialize, Serialize)]
struct User {
    nslist: Vec<Namespace>,
}


impl Component for TestComp {
    type Message = TestMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            nslist: vec![]
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TestMsg::TestClick => {
                js::alert("按钮点击");
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
            <NameSpaceSelect />
            <button onclick={ctx.link().callback(|_| TestMsg::TestClick)}>{"点我"}  </button>
            </div>
            }
    }
}