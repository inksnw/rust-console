use gloo_net::http::Request;
use k8s_openapi::api::core::v1::Namespace;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures;
use yew::prelude::*;

use crate::apis::test::{load_ns_future, TestMsg};
use crate::helper::js;

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
        ctx.link().send_message(TestMsg::LoadNS);
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
            TestMsg::LoadNS => {
                ctx.link().send_future(load_ns_future());
                true
            }
            TestMsg::LoadNSDone(data) => {
                self.nslist = data;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
             <select>
              {for self.nslist.iter().map(|ns:&Namespace| html!{
                <option value={ns.metadata.name.as_ref().unwrap().clone()}>
                {ns.metadata.name.as_ref().unwrap().clone()}
                </option>
              })}
            </select>
            <button onclick={ctx.link().callback(|_| TestMsg::TestClick)}>{"点我"}  </button>
            </div>
            }
    }
}