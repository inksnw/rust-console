use yew::{Context, Html, html};
use yew::prelude::Component;
use yew_router::prelude::*;
use yew_router::scope_ext::HistoryHandle;

use crate::apis::app::{AppMsg, load_pods_future};
use crate::element_ui::table::{ElTable, ElTableColumn, ElTableLink};
use crate::helper::pagination::{PageQuery, Pagination};
use crate::helper::router::Route;

use super::selectns::NameSpaceSelect;

const ITEMS_PER_PAGE: u64 = 5;

fn current_page(ctx: &Context<Pods>) -> u64 {
    let location = ctx.link().location().unwrap();
    location.query::<PageQuery>().map(|it| it.page).unwrap_or(1)
}


pub struct Pods {
    ns: String,
    pods: Vec<serde_json::Value>,
    page: u64,
    total_items: u64,
    _listener: HistoryHandle,
}

impl Component for Pods {
    type Message = AppMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(load_pods_future(None, None, Some("1".to_string()), "pods".to_string()));
        let link = ctx.link().clone();
        let listener = ctx
            .link()
            .add_history_listener(link.callback(move |_| AppMsg::PageUpdated))
            .unwrap();

        Self {
            ns: String::new(),
            pods: vec![],
            page: current_page(ctx),
            total_items: 1,
            _listener: listener,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::UpdateNs(newvalue) => {
                self.ns = newvalue.value;
                self.page = 1;
                ctx.link().send_future(load_pods_future(Some(self.ns.to_string()), None, Some("1".to_string()), "pods".to_string()));
            }
            AppMsg::LoadPodsDone(pods_str) => {
                let tmp: serde_json::Value = serde_json::from_str(pods_str.as_str()).unwrap();
                let tmp1 = tmp.get("items").unwrap().to_string();

                let total_items = tmp.get("total_items").unwrap().to_string();
                self.pods = serde_json::from_str(&tmp1[..]).unwrap();
                self.total_items = total_items.parse().unwrap();
            }
            AppMsg::PageUpdated => {
                self.page = current_page(ctx);
                ctx.link().send_future(load_pods_future(None, None, Some(self.page.to_string()), "pods".to_string()));
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let page = self.page;
        let total_pages = (self.total_items + ITEMS_PER_PAGE - 1) / ITEMS_PER_PAGE;
        let total_pages = if total_pages == 0 { 1 } else { total_pages };

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
            <Pagination {page} total_pages={total_pages} route_to_page={Route::Pods}/>
            </div>
        }
    }
}



