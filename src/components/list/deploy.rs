use serde_json::Value;
use yew::{Context, Html, html};
use yew::prelude::Component;
use yew_agent::{Bridge, Bridged};
use yew_router::prelude::RouterScopeExt;
use yew_router::scope_ext::HistoryHandle;

use crate::apis::app::AppMsg;
use crate::components::list::base;
use crate::components::list::base::Updatable;
use crate::components::selectns::NameSpaceSelect;
use crate::element_ui::table::{ElTable, ElTableColumn, ElTableLink};
use crate::helper::event_bus::EventBus;
use crate::helper::message::success;
use crate::helper::modal::Modal;
use crate::helper::pagination::Pagination;
use crate::helper::router::Route;

pub const ITEMS_PER_PAGE: u64 = 5;

pub struct Deploy {
    pub ns: Option<String>,
    pub data: Vec<Value>,
    pub page: u64,
    pub total_items: u64,
    pub(crate) _listener: HistoryHandle,
    _producer: Box<dyn Bridge<EventBus>>,
}

impl Updatable for Deploy {
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

impl Component for Deploy {
    type Message = AppMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            ns: None,
            data: vec![],
            page: base::current_page::<Deploy>(ctx),
            total_items: 1,
            _listener: base::gen_listener::<Deploy>(ctx, "deployments".to_string()),
            _producer: EventBus::bridge(ctx.link().callback(AppMsg::HandleMsg)),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        Updatable::update(self, ctx, msg, "deployments".to_string())
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let page = self.page();
        let total_pages = (self.total_items() + ITEMS_PER_PAGE - 1) / ITEMS_PER_PAGE;
        let total_pages = if total_pages == 0 { 1 } else { total_pages };
        let history = ctx.link().history().unwrap();
        let mm = "hello".to_string();
        success(&mm);
        // class="el-popup-parent--hidden"
        // <div class="v-modal" tabindex="0" style="z-index: 2086;"></div>

        html!(
            <div>
            <Modal visible={false}/>
            {base::render_workload_nav("deploy".to_string())}
            <NameSpaceSelect onchange={ctx.link().callback(AppMsg::UpdateNs)} />
            <ElTable width={"100%"} data={self.data.clone()}  history={history}>
            <ElTableColumn label="??????" prop="metadata.name" width="200"/>
            <ElTableColumn label="????????????" prop="metadata.namespace" width="200"/>
            <ElTableColumn label="????????????" prop="metadata.creationTimestamp" width="200"/>
            <ElTableColumn label="????????????" prop="metadata.ownerReferences.0.kind" width="200"/>
            <ElTableColumn label="????????????" prop="metadata.ownerReferences.0.name" width="200"/>
            <ElTableColumn label="??????pod???" prop="status.availableReplicas" width="100"/>
            <ElTableColumn label="??????pod???" prop="spec.replicas" width="100"/>
             <ElTableColumn label="??????">
                <ElTableLink href={"/a?name=$1&ns=$2"} params={vec!("metadata.name","metadata.namespace")} label="??????"/>
            </ElTableColumn>
            </ElTable>
            <Pagination {page} total_pages={total_pages} route_to_page={Route::Deploy}/>
            </div>
        )
    }
}

