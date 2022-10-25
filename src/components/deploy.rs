
use yew::{Context, Html, html};
use yew::prelude::Component;

use crate::apis::app::{AppMsg, load_pods_future};
use crate::element_ui::table::ElTable;
use crate::element_ui::table::ElTableColumn;

use super::selectns::NameSpaceSelect;

pub struct Deploy {
    ns: String,
    pods: Vec<serde_json::Value>,
}


impl Component for Deploy {
    type Message = AppMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(load_pods_future(None,"deployments".to_string()));
        Self {
            ns: String::new(),
            pods: vec![],
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::UpdateNs(newvalue) => {
                self.ns = newvalue.value;
                ctx.link().send_future(load_pods_future(Some(self.ns.to_string()),"deployments".to_string()));
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
            <ElTable width={"100%"} data={self.pods.clone()}>
            <ElTableColumn label="名称" prop="metadata.name" width="200"/>
            <ElTableColumn label="名称空间" prop="metadata.namespace" width="200"/>
            </ElTable>
            </div>
        }
    }
}


