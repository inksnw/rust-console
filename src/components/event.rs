use yew::{Context, Html, html, Properties};
use yew::prelude::Component;
use yew_router::prelude::*;

use crate::apis::app::{AppMsg, load_data_future};
use crate::element_ui::table::ElTable;
use crate::element_ui::table::ElTableColumn;
use crate::helper::router::Route;

pub struct Event {
    pods: Vec<serde_json::Value>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct EventProp {
    pub ns: String,
    pub name: String,
}

impl Component for Event {
    type Message = AppMsg;
    type Properties = EventProp;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(load_data_future(Some(ctx.props().ns.to_string()), None, None, "events".to_string()));
        Self {
            pods: vec![],
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::UpdateNs(_) => {}
            AppMsg::LoadDataDone(pods_str) => {
                let tmp: serde_json::Value = serde_json::from_str(pods_str.as_str()).unwrap();
                let tmp1 = tmp.get("items").unwrap().to_string();
                self.pods = serde_json::from_str(&tmp1[..]).unwrap();
            }
            _ => {}
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!(
            <div>

            <ul role="menubar" class="el-menu-demo el-menu--horizontal el-menu">
            <li role="menuitem" tabindex="0" class="el-menu-item">
             <Link<Route>  to={Route::PodDetail { ns:   ctx.props().ns.clone(), id:   ctx.props().name.clone() } }>{ {"资源状态"} }</Link<Route>>
            </li>
            <li role="menuitem" tabindex="0" class="el-menu-item is-active">
             { {"事件"} }
            </li>
            </ul>
           <div>
            <ElTable width={"100%"} data={self.pods.clone()}>
            <ElTableColumn label="时间" prop="firstTimestamp" width="200"/>
            <ElTableColumn label="类型" prop="type"/>
            <ElTableColumn label="消息" prop="message" width="200"/>
            </ElTable>
            </div>
            </div>
        )
    }
}



