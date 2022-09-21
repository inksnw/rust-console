use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures;
use yew::prelude::*;

use crate::helper::js;

pub struct TestComp {
    pub name: String,
}

pub enum Msg {
    TestClick,
    LoadUser,
    LoadUserDone(Option<String>),
}

#[derive(Clone, PartialEq, Deserialize, Serialize)]
struct User {
    message: String,
}

async fn get_user() -> Result<String, wasm_bindgen::JsValue> {
    let get_body: User = Request::get("http://localhost:8081/test")
        .send().await.unwrap()
        .json().await.unwrap();
    Ok(get_body.message.to_string())
}

impl Component for TestComp {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::LoadUser);
        Self {
            name: String::from("haheeeeea")
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::TestClick => {
                js::alert("按钮点击");
                true
            }
            Msg::LoadUser => {
                let f = async {
                    match get_user().await {
                        Ok(u) => Msg::LoadUserDone(Some(u)),
                        Err(_) => Msg::LoadUserDone(Some("no data".to_string()))
                    }
                };
                ctx.link().send_future(f);
                true
            }
            Msg::LoadUserDone(data) => {
                if let Some(u) = data {
                    self.name = u;
                }
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