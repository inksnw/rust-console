use k8s_openapi::api::core::v1::Namespace;
use yew::prelude::*;
use yew::Properties;

use crate::apis::namespace::*;

#[derive(Properties, PartialEq)]
pub struct NamespaceProps {
    #[prop_or(false)]
    pub disabled: bool,
}

pub struct NameSpaceSelect {
    nslist: Vec<Namespace>,
}

impl Component for NameSpaceSelect {
    type Message = NamespaceMsg;
    type Properties = NamespaceProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(NamespaceMsg::LoadNS);
        Self { nslist: vec![] }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            NamespaceMsg::LoadNS => {
                ctx.link().send_future(load_ns_future());
                true
            }
            NamespaceMsg::LoadNSDone(data) => {
                self.nslist = data;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
        <div>
        <select disabled={ctx.props().disabled}>
        {
            for self.nslist.iter().map(|ns:&Namespace|html!{
                <option value={ns.metadata.name.as_ref().unwrap().clone()}>
                {ns.metadata.name.as_ref().unwrap().clone()}
                </option>
        })
        }
        </select>
        </div>
        }
    }
}