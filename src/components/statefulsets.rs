use serde_json::Value;
use yew::{Context, Html, html};
use yew::prelude::Component;
use yew_router::scope_ext::HistoryHandle;

use crate::apis::app::AppMsg;
use crate::components::base::{current_page, gen_listener, render_workload_nav, Updatable};
use crate::element_ui::table::{ElTable, ElTableColumn, ElTableLink};
use crate::helper::pagination::Pagination;
use crate::helper::router::Route;

use super::selectns::NameSpaceSelect;

pub const ITEMS_PER_PAGE: u64 = 5;


pub struct StateFulSets {
    pub ns: Option<String>,
    pub data: Vec<Value>,
    pub page: u64,
    pub total_items: u64,
    pub(crate) _listener: HistoryHandle,
}


impl Updatable for StateFulSets {
    fn ns(&self) -> Option<String> {
        self.ns.clone()
    }
    fn page(&self) -> u64 { self.page }
    fn update_data(&mut self, data: Vec<Value>) {
        self.data = data
    }
    fn update_ns(&mut self, value: Option<String>) {
        self.ns = value
    }
    fn update_page(&mut self, page: u64) {
        self.page = page
    }
    fn update_total_item(&mut self, total_item: u64) {
        self.total_items = total_item
    }
    fn total_items(&self) -> u64 {
        self.total_items
    }
}

impl Component for StateFulSets {
    type Message = AppMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            ns: None,
            data: vec![],
            page: current_page::<StateFulSets>(ctx),
            total_items: 1,
            _listener: gen_listener::<StateFulSets>(ctx, "statefulsets".to_string()),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        Updatable::update(self, ctx, msg, "statefulsets".to_string())
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let page = self.page();
        let total_pages = (self.total_items() + ITEMS_PER_PAGE - 1) / ITEMS_PER_PAGE;
        let total_pages = if total_pages == 0 { 1 } else { total_pages };

        html! {
            <div>
              {render_workload_nav()}
            <NameSpaceSelect onchange={ctx.link().callback(AppMsg::UpdateNs)} />
            <ElTable width={"100%"} data={self.data.clone()}>
            <ElTableColumn label="名称" prop="metadata.name" width="200"/>
            <ElTableColumn label="创建时间" prop="metadata.creationTimestamp" width="200"/>
             <ElTableColumn label="操作">
                <ElTableLink href={"/a?name=$1&ns=$2"} params={vec!("metadata.name","metadata.namespace")} label="删除"/>
            </ElTableColumn>
            </ElTable>
            <Pagination {page} total_pages={total_pages} route_to_page={Route::StateFulSets}/>
            </div>
        }
    }
}