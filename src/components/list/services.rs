use serde_json::Value;
use yew::{Context, Html, html};
use yew::prelude::Component;
use yew_router::prelude::RouterScopeExt;
use yew_router::scope_ext::HistoryHandle;

use crate::apis::app::AppMsg;
use crate::components::selectns::NameSpaceSelect;
use crate::element_ui::table::{ElTable, ElTableColumn, ElTableLink};
use crate::helper::pagination::Pagination;
use crate::helper::router::Route;

use super::base::{self, Updatable};

pub const ITEMS_PER_PAGE: u64 = 5;


pub struct Services {
    pub ns: Option<String>,
    pub data: Vec<Value>,
    pub page: u64,
    pub total_items: u64,
    pub(crate) _listener: HistoryHandle,
}


impl Updatable for Services {
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

impl Component for Services {
    type Message = AppMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            ns: None,
            data: vec![],
            page: base::current_page::<Services>(ctx),
            total_items: 1,
            _listener: base::gen_listener::<Services>(ctx, "services".to_string()),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        Updatable::update(self, ctx, msg, "services".to_string())
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let page = self.page();
        let total_pages = (self.total_items() + ITEMS_PER_PAGE - 1) / ITEMS_PER_PAGE;
        let total_pages = if total_pages == 0 { 1 } else { total_pages };

        let history = ctx.link().history().unwrap();
        html! {
            <div>
            <NameSpaceSelect onchange={ctx.link().callback(AppMsg::UpdateNs)} />
            <ElTable width={"100%"} data={self.data.clone()} history={history}>
            <ElTableColumn label="??????" prop="metadata.name" width="200"/>
            <ElTableColumn label="????????????" prop="metadata.creationTimestamp" width="200"/>
            <ElTableColumn label="????????????" prop="metadata.ownerReferences.0.kind" width="200"/>
            <ElTableColumn label="????????????" prop="metadata.ownerReferences.0.name" width="200"/>
              <ElTableColumn label="clusterIP" prop="spec.clusterIP" width="200"/>
              <ElTableColumn label="ports" prop="spec.ports.0.port" width="200"/>
              <ElTableColumn label="type" prop="spec.type" width="200"/>
             <ElTableColumn label="??????">
                <ElTableLink href={"/a?name=$1&ns=$2"} params={vec!("metadata.name","metadata.namespace")} label="??????"/>
            </ElTableColumn>
            </ElTable>
            <Pagination {page} total_pages={total_pages} route_to_page={Route::Services}/>
            </div>
        }
    }
}