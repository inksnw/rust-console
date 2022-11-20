use serde_json::Value;
use yew::{Context, Html, html};
use yew::prelude::Component;
use yew::Properties;
use yew_router::prelude::*;

use crate::apis::app::{AppMsg, load_data_future};
use crate::helper::router::Route;
use crate::helper::utils::{get_json_value, value2string};

pub struct DeployDetail {
    data: Value,
}

#[derive(Clone, PartialEq, Properties)]
pub struct DeployDetailProp {
    pub ns: String,
    pub name: String,
}

impl Component for DeployDetail {
    type Message = AppMsg;
    type Properties = DeployDetailProp;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(load_data_future(Some(ctx.props().ns.to_string()), Some(ctx.props().name.to_string()), None, "deployments".to_string()));
        Self {
            data: Value::String(String::new()),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::LoadDataDone(data) => {
                self.data = serde_json::from_str(data.as_str()).unwrap();
            }
            _ => {}
        }
        true
    }


    fn view(&self, ctx: &Context<Self>) -> Html {
        html!(
            <div>
            <ul role="menubar" class="el-menu-demo el-menu--horizontal el-menu">
            <li role="menuitem" tabindex="0" class="el-menu-item is-active">{{"资源状态"}}</li>
            <li role="menuitem" tabindex="1" class="el-menu-item">
            <Link<Route>  to={Route::Event{ns:ctx.props().ns.clone(),id:ctx.props().name.clone()}}>
                { {"事件"} }
            </Link<Route>>
            </li>
            </ul>
            { self.info()}
            </div>
        )
    }
}


impl DeployDetail {
    fn td(&self, k: String, v: String) -> Html {
        html!(
            <td colspan="1" class="el-descriptions-item el-descriptions-item__cell">
                <div class="el-descriptions-item__container">
                    <span class="el-descriptions-item__label has-colon ">{k}</span>
                    <span class="el-descriptions-item__content">{v}</span>
                </div>
            </td>
        )
    }

    fn info(&self) -> Html {
        let empty_value = Value::String(String::new());
        let name = value2string(get_json_value("metadata.name", &self.data, &empty_value));
        let create_time = value2string(get_json_value("metadata.creationTimestamp", &self.data, &empty_value));

        html! {
            <div class="el-descriptions">
                <div class="el-descriptions__body">
                    <table class="el-descriptions__table">
                        <tbody>
                        <tr class="el-descriptions-row">
                            {self.td("名称".to_string(),name)}
                            {self.td("创建时间".to_string(),create_time)}
                        </tr>
                        </tbody>
                    </table>
                </div>
            </div>
    }
    }
}
