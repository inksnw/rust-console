use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures;
use yew::prelude::*;

use crate::helper::js;

pub struct TestComp {
    pub name: String,
}

pub enum Msg {
    TestClick
}

#[derive(Clone, PartialEq, Deserialize, Serialize)]
struct User {
    message: String,
}

impl Component for TestComp {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        wasm_bindgen_futures::spawn_local(async move {
            let get_body: User = Request::get("http://localhost:8081/test")
                .send().await.unwrap()
                .json().await.unwrap();
            js::log(&wasm_bindgen::JsValue::from_serde(&get_body).unwrap());
        });
        Self {
            name: String::from("haheeeeea")
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