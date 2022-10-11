use k8s_openapi::api::core::v1::Namespace;
use yew::prelude::*;
use yew::Properties;

use crate::apis::namespace::*;
use crate::element_ui::select::ElSelect;
use crate::element_ui::ValueText;

#[derive(Properties, PartialEq)]
pub struct NamespaceProps {
    #[prop_or(false)]
    pub disabled: bool,
    pub onchange: Option<Callback<ValueText>>,
}

pub struct NameSpaceSelect {
    ns_list: Vec<Namespace>,
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
                ctx.link().send_future(load_ns_future());
            }
            NamespaceMsg::LoadNSDone(data) => {
                self.ns_list = data;
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
        html! {
          <div>
          {self.render_select(ctx)}
          </div>
      }
    }
}

impl NameSpaceSelect {
    fn render_select(&self, ctx: &Context<Self>) -> Html {
        let nsdata = self.ns_list.iter().
            map(|ns: &Namespace| ValueText {
                value: ns.metadata.name.as_ref().unwrap().clone(),
                text: ns.metadata.name.as_ref().unwrap().clone(),
            }).collect::<Vec<ValueText>>();
        html! {
        <ElSelect data={nsdata} onchange={ctx.link().callback(NamespaceMsg::Onchange)} />
      }
    }
}