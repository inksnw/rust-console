use serde_json::Value;
use yew::Context;
use yew::prelude::Component;
use yew_router::prelude::*;
use yew_router::scope_ext::HistoryHandle;

use crate::apis::app::{AppMsg, load_pods_future};
use crate::helper::pagination::PageQuery;

pub fn current_page<T: Component>(ctx: &Context<T>) -> u64 {
    let location = ctx.link().location().unwrap();
    location.query::<PageQuery>().map(|it| it.page).unwrap_or(1)
}

pub fn gen_listener<T>(ctx: &Context<T>, name: String) -> HistoryHandle
    where <T as Component>::Message: From<AppMsg>, T: Component {
    ctx.link().send_future(load_pods_future(None, None, Some("1".to_string()), name));
    let link = ctx.link().clone();
    let listener = ctx
        .link()
        .add_history_listener(link.callback(move |_| AppMsg::PageUpdated))
        .unwrap();
    listener
}


pub trait Updatable<T = Self>
    where T: Component, <T as Component>::Message: From<AppMsg>
{
    fn ns(&self) -> Option<String>;
    fn page(&self) -> u64;
    fn update_pods(&mut self, pods: Vec<Value>);
    fn update_ns(&mut self, value: Option<String>);
    fn update_page(&mut self, page: u64);
    fn update_total_item(&mut self, total_item: u64);
    fn total_items(&self) -> u64;

    fn update(&mut self, ctx: &Context<T>, msg: AppMsg, name: String) -> bool {
        match msg {
            AppMsg::UpdateNs(value) => {
                self.update_ns(Some(value.value));
                self.update_page(1);
                ctx.link().send_future(load_pods_future(self.ns().clone(), None, Some("1".to_string()), name));
            }
            AppMsg::LoadPodsDone(pods_str) => {
                let tmp: serde_json::Value = serde_json::from_str(pods_str.as_str()).unwrap();
                let tmp1 = tmp.get("items").unwrap().to_string();

                let total_items = tmp.get("total_items").unwrap().to_string();
                self.update_pods(serde_json::from_str(&tmp1[..]).unwrap());
                self.update_total_item(total_items.parse().unwrap());
            }
            AppMsg::PageUpdated => {
                self.update_page(current_page::<T>(ctx));
                ctx.link().send_future(load_pods_future(self.ns().clone(), None, Some(self.page().to_string()), "pods".to_string()));
            }
        }
        true
    }
}
