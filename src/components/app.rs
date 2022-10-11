use gloo::console::log;
use serde::{Deserialize, Serialize};
use stylist::Style;
use yew::{Context, Html, html};
use yew::prelude::Component;

use crate::apis::app::{AppMsg, load_pods_future};
use crate::element_ui::input::ElInput;
use crate::element_ui::table::ElTable;
use crate::element_ui::table::ElTableColumn;
use crate::helper::js;

use super::selectns::NameSpaceSelect;

#[allow(dead_code)]
pub struct Resource {
    myname: String,
    ns: String,
    pods: Vec<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
struct MyObject {
    user_name: String,
    age: u32,
}

const STYLE: &str = include_str!("main.css");

impl Component for Resource {
    type Message = AppMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(load_pods_future(None));
        Self {
            myname: String::from("test_name"),
            ns: String::new(),
            pods: vec![],
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::TestClick => {
                js::alert("按钮点击");
            }
            AppMsg::UpdateMyName(newvalue) => {
                self.myname = newvalue;
            }
            AppMsg::UpdateNs(newvalue) => {
                self.ns = newvalue.value;
                ctx.link().send_future(load_pods_future(Some(self.ns.to_string())));
            }
            AppMsg::LoadPodsDone(pods_str) => {
                self.pods = serde_json::from_str(pods_str.as_str()).unwrap();
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
            <NameSpaceSelect onchange={ctx.link().callback(AppMsg::UpdateNs)} />
            <ElInput value={self.myname.clone()} onchange={ctx.link().callback(AppMsg::UpdateMyName)} />
            <button onclick={ctx.link().callback(|_| AppMsg::TestClick)}>{"点我"}  </button>

            <ElTable width={"100%"} data={self.pods.clone()}>
            <ElTableColumn label="pod名" prop="metadata.name" width="200"/>
            <ElTableColumn label="名称空间" prop="metadata.namespace" width="200"/>
            <ElTableColumn label="状态" prop="status.phase"/>
            <ElTableColumn label="IP" prop="status.podIP" width="200"/>
            </ElTable>
            </div>
        }
    }
}



