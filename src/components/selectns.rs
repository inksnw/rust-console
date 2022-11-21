use serde_json::Value;
use yew::prelude::*;
use yew::Properties;

use crate::apis::apiv1::*;
use crate::element_ui::ElButton;
use crate::element_ui::select::ElSelect;
use crate::element_ui::ValueText;
use crate::helper::utils;

#[derive(Properties, PartialEq)]
pub struct NamespaceProps {
    #[prop_or(false)]
    pub disabled: bool,
    pub onchange: Option<Callback<ValueText>>,
}

pub struct NameSpaceSelect {
    ns_list: Vec<Value>,
}

impl Component for NameSpaceSelect {
    type Message = NamespaceMsg;
    type Properties = NamespaceProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(NamespaceMsg::LoadNS);
        Self { ns_list: vec![] }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            NamespaceMsg::LoadNS => {
                ctx.link().send_future(load_ns_future("namespaces".to_string()));
            }
            NamespaceMsg::LoadNSDone(data) => {
                let tmp: serde_json::Value = serde_json::from_str(data.as_str()).unwrap();
                let tmp1 = tmp.get("items").unwrap().to_string();
                self.ns_list = serde_json::from_str(&tmp1[..]).unwrap();
            }
            NamespaceMsg::Onchange(vt) => {
                if let Some(call) = ctx.props().onchange.clone() {
                    call.emit(vt);
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!(
          <div>
          {self.render_select(ctx)}
          </div>
        )
    }
}

impl NameSpaceSelect {
    fn render_select(&self, ctx: &Context<Self>) -> Html {
        let empty_value = Value::String(String::new());
        let nsdata = self.ns_list.iter().
            map(|item| ValueText {
                value: utils::value2string(utils::get_json_value("metadata.name", item, &empty_value)),
                text: utils::value2string(utils::get_json_value("metadata.name", item, &empty_value)),
            }).collect::<Vec<ValueText>>();
        html! {
            <div>
            <div style="display:inline;">
            <ElSelect data={nsdata} onchange={ctx.link().callback(NamespaceMsg::Onchange)} />
            </div>
            <div  style="display:inline;float:right;">
            <ElButton value={"创建"} button_type={"primary"}/>
            </div>
            </div>
      }
    }
}