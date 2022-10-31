use yew::{Context, Html, html};
use yew::prelude::Component;

use crate::apis::app::{AppMsg, load_pods_future};
use crate::element_ui::table::{ElTable, ElTableColumn, ElTableLink};
use crate::helper::pagination::Pagination;
use crate::helper::router::Route;

use super::selectns::NameSpaceSelect;

const ITEMS_PER_PAGE: u64 = 10;
const TOTAL_PAGES: u64 = 100 / ITEMS_PER_PAGE;

pub struct Pods {
    page: u64,
    ns: String,
    pods: Vec<serde_json::Value>,
}

impl Component for Pods {
    type Message = AppMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(load_pods_future(None, None, "pods".to_string()));
        Self {
            page: 1,
            ns: String::new(),
            pods: vec![],
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::UpdateNs(newvalue) => {
                self.ns = newvalue.value;
                ctx.link().send_future(load_pods_future(Some(self.ns.to_string()), None, "pods".to_string()));
            }
            AppMsg::LoadPodsDone(pods_str) => {
                self.pods = serde_json::from_str(pods_str.as_str()).unwrap();
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let page = self.page;
        html! {
            <div>
            <NameSpaceSelect onchange={ctx.link().callback(AppMsg::UpdateNs)} />
            <ElTable width={"100%"} data={self.pods.clone()}>
            <ElTableColumn label="pod名" prop="metadata.name" width="200"/>
            <ElTableColumn label="名称空间" prop="metadata.namespace" width="200"/>
            <ElTableColumn label="状态" prop="status.phase"/>
            <ElTableColumn label="IP" prop="status.podIP" width="200"/>
             <ElTableColumn label="操作">
                <ElTableLink href={"/a?name=$1&ns=$2"} params={vec!("metadata.name","metadata.namespace")} label="删除"/>
            </ElTableColumn>
            </ElTable>
            <Pagination {page} total_pages={TOTAL_PAGES} route_to_page={Route::Pods}/>
            </div>
        }
    }
}



