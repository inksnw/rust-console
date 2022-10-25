use yew::{Context, Html, html};
use yew::prelude::Component;

use crate::apis::apiv1::{load_ns_future, NamespaceMsg};
use crate::element_ui::table::ElTable;
use crate::element_ui::table::ElTableColumn;

pub struct Nodes {
    nodes: Vec<serde_json::Value>,
}

impl Component for Nodes {
    type Message = NamespaceMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(load_ns_future("nodes".to_string()));
        Self {
            nodes: vec![],
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            NamespaceMsg::LoadNS => {
                ctx.link().send_future(load_ns_future("namespaces".to_string()));
            }
            NamespaceMsg::LoadNSDone(pods_str) => {

                self.nodes = serde_json::from_str(pods_str.as_str()).unwrap();
            }
            _ => {}
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>

            <ElTable width={"100%"} data={self.nodes.clone()}>
            <ElTableColumn label="名称" prop="metadata.name" width="200"/>
            <ElTableColumn label="IP" prop="status.addresses.0.address" width="200"/>
            <ElTableColumn label="架构" prop="status.nodeInfo.architecture" width="200"/>
            <ElTableColumn label="系统" prop="status.nodeInfo.osImage" width="200"/>
            </ElTable>
            </div>
        }
    }
}



