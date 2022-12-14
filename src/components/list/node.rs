use yew::{Context, Html, html};
use yew::prelude::Component;
use yew_router::prelude::RouterScopeExt;

use crate::apis::apiv1::{load_ns_future, NamespaceMsg};
use crate::element_ui::table::{ElTable, ElTableColumn};

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
                let tmp: serde_json::Value = serde_json::from_str(pods_str.as_str()).unwrap();
                let tmp1 = tmp.get("items").unwrap().to_string();
                self.nodes = serde_json::from_str(&tmp1[..]).unwrap();
            }
            _ => {}
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let history = ctx.link().history().unwrap();
        html!(
            <div>

            <ElTable width={"100%"} data={self.nodes.clone()} history={history}>
            <ElTableColumn label="名称" prop="metadata.name" width="100"/>
            <ElTableColumn label="IP" prop="status.addresses.0.address" width="120"/>
            <ElTableColumn label="cpu" prop="status.capacity.cpu" width="100"/>
            <ElTableColumn label="内存" prop="status.capacity.memory" width="100"/>
            <ElTableColumn label="最大Pod数" prop="status.capacity.pods" width="100"/>
            <ElTableColumn label="架构" prop="status.nodeInfo.architecture" width="100"/>
            <ElTableColumn label="系统" prop="status.nodeInfo.osImage" width="200"/>
            <ElTableColumn label="容器类型" prop="status.nodeInfo.containerRuntimeVersion" width="200"/>
            </ElTable>
            </div>
        )
    }
}



