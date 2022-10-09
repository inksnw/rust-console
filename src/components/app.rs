use gloo::console::log;
use k8s_openapi::api::core::v1::Namespace;
use serde::{Deserialize, Serialize};
use yew::{Context, Html, html};
use yew::prelude::Component;

use crate::apis::test::TestMsg;
use crate::element_ui::base::ElInput;
use crate::helper::js;

use super::selectns::NameSpaceSelect;

#[allow(dead_code)]
pub struct TestComp {
    ns_list: Vec<Namespace>,
}

#[derive(Serialize, Deserialize)]
struct MyObject {
    user_name: String,
    age: u32,
}

impl Component for TestComp {
    type Message = TestMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            ns_list: vec![]
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TestMsg::TestClick => {
                js::alert("按钮点击");
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let name = "haha";
        log!(name);
        let my_obj = MyObject {
            user_name: name.to_owned(),
            age: 12,
        };
        log!(serde_json::to_string_pretty(&my_obj).unwrap());
        let class_name = "my_title";
        let items = vec!["xiao hong", "xiao ming"];


        html! {
            <div>
            <ul class="item-list">
            {  list_to_html(items) }
            </ul>
            <h1 class="title">{"这是一个标题"}</h1>
            if class_name=="my_title"{
                <p>{"hi there"}</p>
            }
            <NameSpaceSelect />
            <ElInput value="abc"/>
            <button onclick={ctx.link().callback(|_| TestMsg::TestClick)}>{"点我"}  </button>
            </div>
        }
    }
}

fn list_to_html(list: Vec<&str>) -> Vec<Html> {
    list.iter().map(|item| html! {<li>{item}</li>}).collect()
}