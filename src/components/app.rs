use gloo::console::log;
use k8s_openapi::api::core::v1::Namespace;
use serde::{Deserialize, Serialize};
use stylist::Style;
use yew::{Context, Html, html};
use yew::prelude::Component;

use crate::apis::app::AppMsg;
use crate::element_ui::el_input::ElInput;
use crate::helper::js;

use super::selectns::NameSpaceSelect;

#[allow(dead_code)]
pub struct TestComp {
    myname: String,
    ns: String,
}

#[derive(Serialize, Deserialize)]
struct MyObject {
    user_name: String,
    age: u32,
}

const STYLE: &str = include_str!("main.css");

impl Component for TestComp {
    type Message = AppMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            myname: String::from("test_name"),
            ns: String::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::TestClick => {
                js::alert("按钮点击");
            }
            AppMsg::UpdateMyName(newvalue) => {
                self.myname = newvalue;
            }
            AppMsg::UpdateNs(newvalue) => {
                self.ns = newvalue.value;
            }
        }
        true
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

        let stylesheet = Style::new(STYLE).unwrap();

        html! {
            <div class={stylesheet}>
            <ul class="item-list">
            {  list_to_html(items) }
            </ul>
            <h1 class="title">{"这是一个标题"}</h1>
            if class_name=="my_title"{
                <p>{"hi there"}</p>
            }
            <NameSpaceSelect />
            <ElInput value={self.myname.clone()}
            onchange={ctx.link().callback(AppMsg::UpdateMyName)}
            />
            <button onclick={ctx.link().callback(|_| AppMsg::TestClick)}>{"点我"}  </button>
            <h3>{"文本框的内容是"} {self.myname.clone()}</h3>


            </div>
        }
    }
}

fn list_to_html(list: Vec<&str>) -> Vec<Html> {
    list.iter().map(|item| html! {<li>{item}</li>}).collect()
}