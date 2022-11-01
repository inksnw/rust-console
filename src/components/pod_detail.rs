use serde_json::Value;
use yew::{Context, Html, html};
use yew::prelude::Component;
use yew::Properties;
use yew_router::prelude::*;

use crate::apis::app::{AppMsg, load_pods_future};
use crate::helper::router::Route;
use crate::helper::utils::get_json_value;

pub struct PodDetail {
    pods: Value,
}

#[derive(Clone, PartialEq, Properties)]
pub struct PodDetailProp {
    pub ns: String,
    pub name: String,
}

impl Component for PodDetail {
    type Message = AppMsg;
    type Properties = PodDetailProp;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(load_pods_future(Some(ctx.props().ns.to_string()), Some(ctx.props().name.to_string()),None, "pods".to_string()));
        Self {
            pods: Value::String(String::new()),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::UpdateNs(_) => {}
            AppMsg::LoadPodsDone(pods_str) => {
                self.pods = serde_json::from_str(pods_str.as_str()).unwrap();
            }
            _ => {}
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let empty_value = Value::String(String::new());
        let name = get_json_value("metadata.name", &self.pods, &empty_value);
        let create_time = get_json_value("metadata.creationTimestamp", &self.pods, &empty_value);

        html! {
            <div>
            <ul role="menubar" class="el-menu-demo el-menu--horizontal el-menu">
            <li role="menuitem" tabindex="0" class="el-menu-item is-active">{{"资源状态"}}</li>
            <li role="menuitem" tabindex="0" class="el-menu-item">
            <Link<Route>  to={Route::Event{ns:ctx.props().ns.clone(),id:ctx.props().name.clone()}}>{ {"事件"} }</Link<Route>>
            </li>
            </ul>
            <h1>{ format!("pod: {} 创建于 {}",name,create_time) }</h1>
            </div>
        }
    }
}



